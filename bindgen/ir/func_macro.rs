//! Function-like C macro to Rust `const fn` translation.
//!
//! This module translates function-like C preprocessor macros into Rust
//! `const fn` declarations. For example:
//!
//! ```c
//! #define ADD(x, y) ((x) + (y))
//! ```
//!
//! becomes:
//!
//! ```rust,ignore
//! pub const fn ADD(x: i64, y: i64) -> i64 {
//!     ((x) + (y))
//! }
//! ```
//!
//! Parameters used as `sizeof` arguments become generic type parameters:
//!
//! ```c
//! #define _IOR(type, nr, size) _IOC(_IOC_READ, (type), (nr), sizeof(size))
//! ```
//!
//! becomes:
//!
//! ```rust,ignore
//! pub const fn _IOR<size>(r#type: u32, nr: u32) -> u32 {
//!     _IOC(_IOC_READ, (r#type), (nr), core::mem::size_of::<size>() as u32)
//! }
//! ```

use crate::clang::ClangToken;
use clang_sys::*;
use proc_macro2::{Delimiter, Group, Ident, Span, TokenStream};
use quote::quote;
use std::collections::{HashMap, HashSet};
use std::fmt;

/// An owned copy of a C token's kind and spelling, for deferred translation.
///
/// `ClangToken` borrows the translation unit and can't outlive the parse
/// phase. This struct captures the data we need so function-like macros
/// can be translated in a post-parse fixpoint pass.
#[derive(Debug, Clone)]
pub(crate) struct OwnedToken {
    /// The token kind (`CXToken_Punctuation`, `CXToken_Identifier`, etc.).
    pub kind: CXTokenKind,
    /// The token spelling as owned bytes.
    pub spelling: Vec<u8>,
}

impl OwnedToken {
    /// Create from a `ClangToken`.
    pub fn from_clang(t: &ClangToken) -> Self {
        Self {
            kind: t.kind,
            spelling: t.spelling().to_vec(),
        }
    }

    /// Get the spelling as a byte slice (matches `ClangToken::spelling()`).
    pub fn spelling(&self) -> &[u8] {
        &self.spelling
    }
}

/// A raw function-like macro definition collected during parse, before
/// translation. Stored on `BindgenContext` for deferred fixpoint translation.
#[derive(Debug, Clone)]
pub(crate) struct RawFunctionMacro {
    /// The macro name.
    pub name: String,
    /// All tokens of the macro definition (including name and param list).
    pub tokens: Vec<OwnedToken>,
    /// The source file this macro was defined in.
    pub source_file: Option<String>,
}

/// Default value type when inference finds nothing.
pub(crate) const DEFAULT_VALUE_TYPE: &str = "i64";

/// Rust keywords that must be escaped with `r#` when used as identifiers.
pub(crate) const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
    "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct",
    "super", "trait", "true", "type", "unsafe", "use", "where", "while",
    "async", "await", "dyn", "abstract", "become", "box", "do", "final",
    "macro", "override", "priv", "typeof", "unsized", "virtual", "yield",
    "try",
];

/// Create a `proc_macro2::Ident`, using raw identifier syntax for Rust
/// keywords.
fn make_ident(name: &str) -> Ident {
    if RUST_KEYWORDS.contains(&name) {
        Ident::new_raw(name, Span::call_site())
    } else {
        Ident::new(name, Span::call_site())
    }
}

/// Escape a name if it's a Rust keyword (for string-level param tracking).
fn escape_keyword(name: &str) -> String {
    if RUST_KEYWORDS.contains(&name) {
        format!("r#{name}")
    } else {
        name.to_owned()
    }
}

/// Parse a string as a `TokenStream`. Panics on invalid input — only use for
/// known-valid fragments like type names from our own maps.
fn parse_ts(s: &str) -> TokenStream {
    s.parse().expect("internal: invalid token fragment")
}

/// Reason a function-like macro was skipped during translation.
#[derive(Debug)]
pub(crate) enum SkipReason {
    /// Empty token list or no name.
    Empty,
    /// Macro uses variadic arguments.
    Variadic,
    /// Macro body contains untranslatable C type keywords.
    TypeKeywordInBody(String),
    /// Macro body contains `sizeof` on an unresolvable argument.
    UntranslatableSizeof,
    /// Macro body uses `#` (stringification) or `##` (token pasting).
    /// These are preprocessor-level operations with no `const fn` equivalent.
    TokenPasting,
    /// Macro body calls a function not yet translated (forward reference).
    /// The fixpoint pass will retry.
    UnknownCallee(String),
    /// Macro body contains `(ident)(expr)` which is either a typedef cast
    /// or a parenthesized function call — neither is expressible in
    /// `const fn`.
    FunctionCallOrCast(String),
    /// Translated body is not a valid Rust expression (e.g., contains
    /// compiler-internal constructs, adjacent identifiers, etc.).
    InvalidExpression,
    /// Macro calls another macro with the wrong number of arguments.
    ArityMismatch(String),
    /// Macro body references a non-const variable (e.g., `extern int x`),
    /// which becomes `static mut` in Rust and can't be used in `const fn`.
    MutableStaticReference(String),
    /// Translated body is empty.
    EmptyBody,
}

impl fmt::Display for SkipReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty or unnamed macro"),
            Self::Variadic => write!(f, "variadic macro"),
            Self::TypeKeywordInBody(kw) => {
                write!(f, "untranslatable C keyword `{kw}` in body")
            }
            Self::UntranslatableSizeof => {
                write!(f, "sizeof with unresolvable argument")
            }
            Self::TokenPasting => {
                write!(f, "uses # or ## (stringification/token pasting)")
            }
            Self::UnknownCallee(name) => {
                write!(f, "calls unknown function `{name}` (may be forward reference)")
            }
            Self::FunctionCallOrCast(name) => {
                write!(
                    f,
                    "contains ({name})(...) which is a typedef cast or \
                     function call, neither expressible in const fn"
                )
            }
            Self::InvalidExpression => {
                write!(f, "translated body is not a valid Rust expression")
            }
            Self::ArityMismatch(detail) => {
                write!(f, "argument count mismatch: {detail}")
            }
            Self::MutableStaticReference(name) => {
                write!(
                    f,
                    "references non-const variable `{name}` (static mut \
                     in Rust, not usable in const fn)"
                )
            }
            Self::EmptyBody => write!(f, "empty body after translation"),
        }
    }
}

/// Summary of a previously-translated function macro, used to inform
/// translation of later macros in the fixpoint pass.
#[derive(Debug, Clone)]
pub(crate) struct PriorFunctionMacro {
    /// The macro name.
    pub name: String,
    /// All parameter names in original C declaration order.
    pub all_param_names: Vec<String>,
    /// Names of type parameters. Empty if no generics.
    pub type_param_names: Vec<String>,
    /// Number of value parameters.
    pub value_param_count: usize,
}

/// A parsed function-like C macro that can be translated to a Rust `const fn`.
#[derive(Debug, Clone)]
pub(crate) struct FunctionMacro {
    /// The macro name.
    pub name: String,
    /// All parameter names in original C declaration order.
    pub all_param_names: Vec<String>,
    /// Value parameters (typed arguments).
    pub params: Vec<String>,
    /// Type parameters (used in `sizeof` — become generics).
    pub type_params: Vec<String>,
    /// The macro body as a `proc_macro2::TokenStream`.
    pub body: TokenStream,
    /// The Rust type for value parameters and return (e.g., `"u32"`, `"i64"`).
    pub value_type: String,
    /// The source file this macro was defined in, if known.
    pub source_file: Option<String>,
}

impl FunctionMacro {
    /// Try to parse a function-like macro from clang tokens.
    ///
    /// `constant_types` maps known constant names to their Rust type strings
    /// (e.g., `"u32"`, `"i64"`). This is used to infer the function's value
    /// type and avoid unnecessary casts.
    ///
    /// If `type_override` is `Some`, that type is used instead of inferring.
    ///
    /// Returns `Err(SkipReason)` if the macro can't be translated.
    /// `prior_macros`: info about previously-parsed function macros, used to
    /// detect sizeof-like calls and reject calls to unknown functions.
    pub fn parse(
        tokens: &[OwnedToken],
        constant_types: &HashMap<String, String>,
        type_override: Option<&str>,
        prior_macros: &[PriorFunctionMacro],
        non_const_vars: &HashSet<String>,
        target_pointer_size: usize,
    ) -> Result<Self, SkipReason> {
        if tokens.is_empty() {
            return Err(SkipReason::Empty);
        }

        let name_token = &tokens[0];
        if name_token.kind != CXToken_Identifier {
            return Err(SkipReason::Empty);
        }
        let name = std::str::from_utf8(name_token.spelling())
            .map_err(|_| SkipReason::Empty)?
            .to_owned();

        let lparen = tokens
            .iter()
            .position(|t| t.kind == CXToken_Punctuation && t.spelling() == b"(")
            .ok_or(SkipReason::Empty)?;

        let rparen =
            find_matching_paren(tokens, lparen).ok_or(SkipReason::Empty)?;

        let mut all_params = Vec::new();
        for token in &tokens[lparen + 1..rparen] {
            if token.kind == CXToken_Identifier {
                let param = std::str::from_utf8(token.spelling())
                    .map_err(|_| SkipReason::Empty)?
                    .to_owned();
                if param != "__VA_ARGS__" {
                    all_params.push(param);
                }
            }
            if token.kind == CXToken_Punctuation && token.spelling() == b"..." {
                return Err(SkipReason::Variadic);
            }
        }

        let body_tokens = &tokens[rparen + 1..];
        if body_tokens.is_empty() {
            return Err(SkipReason::EmptyBody);
        }

        // Sizeof-like: takes only type params, no value params.
        let sizeof_fn_names: Vec<String> = prior_macros
            .iter()
            .filter(|m| {
                !m.type_param_names.is_empty() && m.value_param_count == 0
            })
            .map(|m| m.name.clone())
            .collect();
        let sizeof_params = find_sizeof_params(
            body_tokens,
            &all_params,
            &sizeof_fn_names,
            prior_macros,
        );

        let mut params = Vec::new();
        let mut type_params = Vec::new();
        for p in &all_params {
            let escaped = escape_keyword(p);
            if sizeof_params.contains(p) {
                type_params.push(escaped);
            } else {
                params.push(escaped);
            }
        }

        let value_type = type_override.map_or_else(
            || infer_value_type(body_tokens, &all_params, constant_types),
            |s| s.to_owned(),
        );

        // Validate value_type is a parseable TokenStream. This catches
        // bad input from func_macro_type callbacks without panicking.
        if value_type.parse::<TokenStream>().is_err() {
            return Err(SkipReason::InvalidExpression);
        }

        let ctx = TranslateCtx {
            params: &params,
            type_params: &type_params,
            value_type: &value_type,
            constant_types,
            prior_macros,
            non_const_vars,
            target_pointer_size,
        };
        let body = translate_expr(body_tokens, &ctx)?;

        if body.is_empty() {
            return Err(SkipReason::EmptyBody);
        }

        // Validate the body is a syntactically valid Rust expression.
        // This catches glibc/compiler-internal macros whose bodies
        // contain attribute annotations, token pasting results, or
        // other non-expression constructs that slipped through the
        // token-level checks above.
        match syn::parse2::<syn::Expr>(body.clone()) {
            Err(_) => return Err(SkipReason::InvalidExpression),
            Ok(ref expr) if expr_has_unsupported_construct(expr) => {
                return Err(SkipReason::InvalidExpression);
            }
            Ok(_) => {}
        }

        let all_param_names =
            all_params.iter().map(|p| escape_keyword(p)).collect();

        Ok(FunctionMacro {
            name,
            all_param_names,
            params,
            type_params,
            body,
            value_type,
            source_file: None,
        })
    }
}

/// Context passed through the recursive translator.
struct TranslateCtx<'a> {
    /// Value parameter names.
    params: &'a [String],
    /// Type parameter names (for `sizeof`).
    type_params: &'a [String],
    /// The inferred Rust type (e.g., `"u32"`, `"i64"`).
    value_type: &'a str,
    /// Map of known constant names to Rust type strings.
    constant_types: &'a HashMap<String, String>,
    /// Previously-translated function macros (for callee lookups).
    prior_macros: &'a [PriorFunctionMacro],
    /// Names of non-const variables (extern vars → `static mut` in Rust).
    non_const_vars: &'a HashSet<String>,
    /// Target pointer size in bytes (4 for 32-bit, 8 for 64-bit).
    /// Used for target-aware `long`/`unsigned long` mapping.
    target_pointer_size: usize,
}

impl TranslateCtx<'_> {
    /// Get the value type as a `TokenStream`.
    fn vt(&self) -> TokenStream {
        parse_ts(self.value_type)
    }
}

/// Infer the value type from constants and literals in the body.
///
/// Priority: if float literals are present, returns `"f64"` (since mixing
/// floats with integer types produces type errors in Rust). Otherwise
/// uses the most common type among referenced constants, defaulting to
/// `"i64"`.
fn infer_value_type(
    tokens: &[OwnedToken],
    params: &[String],
    constant_types: &HashMap<String, String>,
) -> String {
    let mut type_counts: HashMap<&str, usize> = HashMap::new();
    let mut has_float_literal = false;
    // Track the minimum unsigned type implied by C literal suffixes:
    // U → u32, UL/ULL → u64. None if no unsigned suffix seen.
    let mut unsigned_hint: Option<&str> = None;

    for token in tokens {
        // Check literal suffixes for type hints.
        if token.kind == CXToken_Literal {
            if let Ok(spelling) = std::str::from_utf8(token.spelling()) {
                let num = strip_c_suffix(spelling);
                if is_float_literal(num) {
                    has_float_literal = true;
                }
                // C unsigned suffix determines minimum unsigned type:
                // ULL → u64, UL → u64, U → u32 (but u64 if value
                // exceeds u32::MAX, matching C's promotion rules).
                let lower = spelling.to_ascii_lowercase();
                if lower.contains('u') {
                    let has_long =
                        lower.contains('l') || lower.ends_with("i64");
                    let suffix_type = if has_long {
                        // UL, ULL, LU, LLU, and MSVC ui64/UI64 → u64.
                        "u64"
                    } else if literal_exceeds_u32(num) {
                        // U without L/LL but value > u32::MAX →
                        // C promotes to unsigned long/long long.
                        "u64"
                    } else {
                        "u32"
                    };
                    if unsigned_hint.is_none() || unsigned_hint == Some("u32") {
                        unsigned_hint = Some(suffix_type);
                    }
                }
            }
            continue;
        }

        if token.kind != CXToken_Identifier {
            continue;
        }
        let Ok(name) = std::str::from_utf8(token.spelling()) else {
            continue;
        };
        if params.iter().any(|p| p == name) {
            continue;
        }
        if let Some(ty) = constant_types.get(name) {
            *type_counts.entry(ty.as_str()).or_insert(0) += 1;
        }
    }

    // Float literals force f64 — integer arithmetic ops on floats
    // are validated during translation (bitwise/modulo/shift rejected).
    if has_float_literal {
        return "f64".to_owned();
    }

    // Infer from constant references.
    let has_constants = !type_counts.is_empty();
    let inferred = type_counts
        .into_iter()
        // Break ties deterministically by lexical type name order
        // (so output doesn't depend on HashMap iteration order).
        .max_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(b.0)))
        .map_or(DEFAULT_VALUE_TYPE, |(ty, _)| ty);

    // Apply unsigned suffix hint: U → u32, UL/ULL → u64.
    if let Some(min_unsigned) = unsigned_hint {
        if !has_constants {
            // No constants referenced — suffix determines the type.
            return min_unsigned.to_owned();
        }
        // Constants referenced — flip signed→unsigned, ensure at
        // least as wide as the suffix minimum.
        let flipped = match inferred {
            "i8" => "u8",
            "i16" => "u16",
            "i32" => "u32",
            "i64" => "u64",
            other => other,
        };
        let width = |t: &str| -> u8 {
            match t {
                "u8" => 1,
                "u16" => 2,
                "u32" => 4,
                "u64" => 8,
                _ => 4,
            }
        };
        return if width(flipped) >= width(min_unsigned) {
            flipped.to_owned()
        } else {
            min_unsigned.to_owned()
        };
    }

    inferred.to_owned()
}

// ---------------------------------------------------------------------------
// Expression translator (recursive descent for ternary)
// ---------------------------------------------------------------------------

/// Translate a C token slice to a Rust `TokenStream`.
fn translate_expr(
    tokens: &[OwnedToken],
    ctx: &TranslateCtx,
) -> Result<TokenStream, SkipReason> {
    if let Some((q_pos, c_pos)) = find_ternary(tokens) {
        let cond_tokens = &tokens[..q_pos];
        let cond = translate_expr(cond_tokens, ctx)?;
        let then_ = translate_expr(&tokens[q_pos + 1..c_pos], ctx)?;
        let else_ = translate_expr(&tokens[c_pos + 1..], ctx)?;

        // A condition is directly bool only if it has a comparison
        // AND no logical ops. Logical ops are translated with `as vt`,
        // making the result integer-typed even if sub-expressions are
        // comparisons (e.g., `(x) > 0 && (x) < 10` → `(bool && bool) as i64`).
        let cond_is_raw_bool = has_top_level_comparison(cond_tokens) &&
            find_top_level_logical_op(cond_tokens).is_none();

        return if cond_is_raw_bool {
            Ok(quote! { if #cond { #then_ } else { #else_ } })
        } else if matches!(ctx.value_type, "f64" | "f32") {
            Ok(quote! { if (#cond) != 0.0 { #then_ } else { #else_ } })
        } else {
            Ok(quote! { if ((#cond) as i64) != 0 { #then_ } else { #else_ } })
        };
    }

    // Logical operators: split on || (lowest precedence) or &&.
    // Each operand is wrapped with != 0 if it doesn't already produce
    // bool, and the result is cast back to the value type (C's && and
    // || return 0 or 1 as int).
    if let Some((op_pos, is_or)) = find_top_level_logical_op(tokens) {
        let left_tokens = &tokens[..op_pos];
        let right_tokens = &tokens[op_pos + 1..];

        let left = translate_expr(left_tokens, ctx)?;
        let right = translate_expr(right_tokens, ctx)?;

        let is_float = matches!(ctx.value_type, "f64" | "f32");
        let left_bool = if expr_tokens_are_boolean(left_tokens) {
            left
        } else if is_float {
            quote! { ((#left) != 0.0) }
        } else {
            quote! { (((#left) as i64) != 0) }
        };
        let right_bool = if expr_tokens_are_boolean(right_tokens) {
            right
        } else if is_float {
            quote! { ((#right) != 0.0) }
        } else {
            quote! { (((#right) as i64) != 0) }
        };

        let vt = ctx.vt();
        // Can't cast bool directly to f64 — go through i64.
        let cast = if matches!(ctx.value_type, "f64" | "f32") {
            quote! { as i64 as #vt }
        } else {
            quote! { as #vt }
        };
        return if is_or {
            Ok(quote! { ((#left_bool || #right_bool) #cast) })
        } else {
            Ok(quote! { ((#left_bool && #right_bool) #cast) })
        };
    }

    translate_tokens(tokens, ctx)
}

/// Find a top-level ternary `?` and its matching `:`.
///
/// Returns `Some((question_pos, colon_pos))` if found.
fn find_ternary(tokens: &[OwnedToken]) -> Option<(usize, usize)> {
    let mut depth = 0i32;
    let mut question_pos = None;
    let mut ternary_depth = 0i32;

    for (i, t) in tokens.iter().enumerate() {
        if t.kind != CXToken_Punctuation {
            continue;
        }
        match t.spelling() {
            b"(" => depth += 1,
            b")" => depth -= 1,
            b"?" if depth == 0 => {
                if question_pos.is_none() {
                    question_pos = Some(i);
                } else {
                    ternary_depth += 1;
                }
            }
            b":" if depth == 0 && question_pos.is_some() => {
                if ternary_depth == 0 {
                    return Some((question_pos.unwrap(), i));
                }
                ternary_depth -= 1;
            }
            _ => {}
        }
    }

    None
}

/// Check if tokens contain a top-level comparison operator, meaning the
/// expression evaluates to `bool` in Rust.
///
/// Only actual comparison operators produce `bool` in Rust. `&&` and `||`
/// require `bool` operands (which integer params aren't), and `!` on
/// integers is bitwise NOT (returns integer, not bool). So we only check
/// for `>`, `<`, `>=`, `<=`, `==`, `!=`.
fn has_top_level_comparison(tokens: &[OwnedToken]) -> bool {
    let mut depth = 0i32;

    for t in tokens {
        if t.kind != CXToken_Punctuation {
            continue;
        }
        match t.spelling() {
            b"(" => depth += 1,
            b")" => depth -= 1,
            b">" | b"<" | b">=" | b"<=" | b"==" | b"!=" if depth == 0 => {
                return true;
            }
            _ => {}
        }
    }

    false
}

/// Find the first top-level `||` or `&&` operator in tokens.
///
/// Prefers `||` (lower precedence) over `&&`. Returns `(position, is_or)`.
/// Parenthesized sub-expressions are skipped.
fn find_top_level_logical_op(tokens: &[OwnedToken]) -> Option<(usize, bool)> {
    let mut depth = 0i32;
    let mut first_or = None;
    let mut first_and = None;

    for (i, t) in tokens.iter().enumerate() {
        if t.kind != CXToken_Punctuation {
            continue;
        }
        match t.spelling() {
            b"(" => depth += 1,
            b")" => depth -= 1,
            b"||" if depth == 0 && first_or.is_none() => {
                first_or = Some(i);
            }
            b"&&" if depth == 0 && first_and.is_none() => {
                first_and = Some(i);
            }
            _ => {}
        }
    }

    // Split on || first (lower precedence), then &&.
    if let Some(pos) = first_or {
        Some((pos, true))
    } else {
        first_and.map(|pos| (pos, false))
    }
}

/// Check if a token slice produces a boolean result when translated.
///
/// Only raw comparison operators WITHOUT logical ops produce `bool`.
/// If the tokens also contain `&&`/`||`, the logical op handler wraps
/// the result with `as vt`, making it integer-typed even when
/// sub-expressions are comparisons (e.g., `x > 0 && y > 0` →
/// `((bool && bool) as i64)`).
fn expr_tokens_are_boolean(tokens: &[OwnedToken]) -> bool {
    has_top_level_comparison(tokens) &&
        find_top_level_logical_op(tokens).is_none()
}

// ---------------------------------------------------------------------------
// Flat token-by-token translator
// ---------------------------------------------------------------------------

/// Translate tokens to a Rust `TokenStream`, handling casts, `sizeof`, and
/// operators.
fn translate_tokens(
    tokens: &[OwnedToken],
    ctx: &TranslateCtx,
) -> Result<TokenStream, SkipReason> {
    let mut out = TokenStream::new();
    let mut i = 0;
    // Track whether the last emitted token was a "value" (identifier,
    // literal, or close-paren). Used to distinguish binary & (bitwise
    // AND, after a value) from unary & (address-of, after an operator).
    let mut prev_was_value = false;

    while i < tokens.len() {
        let token = &tokens[i];
        let spelling = std::str::from_utf8(token.spelling())
            .map_err(|_| SkipReason::Empty)?;

        match token.kind {
            CXToken_Identifier => {
                prev_was_value = true;
                // Reject compiler builtins that pass syn validation
                // (they parse as function calls) but produce
                // uncompilable code.
                if spelling == "__asm__" ||
                    spelling == "__asm" ||
                    spelling.starts_with("__builtin_")
                {
                    return Err(SkipReason::TypeKeywordInBody(
                        spelling.to_owned(),
                    ));
                }

                if spelling == "sizeof" {
                    let (ts, consumed) =
                        translate_sizeof(&tokens[i + 1..], ctx)?;
                    out.extend(ts);
                    prev_was_value = true;
                    i += 1 + consumed;
                    continue;
                }

                let escaped = escape_keyword(spelling);
                let ident = make_ident(spelling);

                if ctx.params.contains(&escaped) {
                    out.extend(quote! { #ident });
                } else {
                    let next_is_call = tokens.get(i + 1).is_some_and(|t| {
                        t.kind == CXToken_Punctuation && t.spelling() == b"("
                    });
                    if next_is_call {
                        // Consult callee metadata for turbofish / type
                        // param routing. Returns None if callee unknown
                        // (forward reference — fixpoint will retry).
                        if let Some((call_ts, consumed)) =
                            try_translate_call(spelling, &tokens[i + 1..], ctx)?
                        {
                            out.extend(call_ts);
                            i += 1 + consumed;
                            continue;
                        }
                        // Unknown callee or no special handling needed.
                        out.extend(quote! { #ident });
                    } else {
                        // Reject references to non-const variables
                        // (extern → static mut in Rust, not usable
                        // in const fn).
                        if ctx.non_const_vars.contains(spelling) {
                            return Err(SkipReason::MutableStaticReference(
                                spelling.to_owned(),
                            ));
                        }

                        let needs_cast = ctx
                            .constant_types
                            .get(spelling)
                            .map_or(true, |ty| ty != ctx.value_type);
                        if needs_cast {
                            let vt = ctx.vt();
                            out.extend(quote! { (#ident as #vt) });
                        } else {
                            out.extend(quote! { #ident });
                        }
                    }
                }
            }

            CXToken_Literal => {
                prev_was_value = true;
                out.extend(translate_literal(spelling, ctx)?);
            }

            CXToken_Punctuation => match spelling {
                // Stringification and token pasting — no const fn
                // equivalent.
                "#" | "##" => {
                    return Err(SkipReason::TokenPasting);
                }
                // Assignment operators — produce `()` in Rust, not a
                // value, so the return type would mismatch.
                "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" |
                "^=" | "<<=" | ">>=" => {
                    return Err(SkipReason::InvalidExpression);
                }
                // Note: && and || are handled at the translate_expr
                // level (recursive descent), so they should not reach
                // here. If they do, it's a bug — reject to be safe.
                "&&" | "||" => {
                    return Err(SkipReason::InvalidExpression);
                }
                // Bitwise, modulo, and shift operators are invalid on
                // floats. Reject when value type is f64/f32.
                "%" | "|" | "^" | "<<" | ">>"
                    if matches!(ctx.value_type, "f64" | "f32") =>
                {
                    return Err(SkipReason::InvalidExpression);
                }
                "&" if matches!(ctx.value_type, "f64" | "f32") => {
                    return Err(SkipReason::InvalidExpression);
                }
                // Unary & (address-of) produces a reference type, not
                // an integer. Detect by checking if the previous token
                // was a value — if not, & is unary (not binary AND).
                "&" if !prev_was_value => {
                    return Err(SkipReason::InvalidExpression);
                }
                "~" if matches!(ctx.value_type, "f64" | "f32") => {
                    return Err(SkipReason::InvalidExpression);
                }
                "~" => {
                    out.extend(quote! { ! });
                }
                "!" => {
                    // C's `!` is logical NOT (0→1, nonzero→0).
                    // Rust's `!` on integers is bitwise NOT.
                    // Translate: !operand → (((operand) as i64 == 0) as vt)
                    let (operand_ts, consumed) =
                        translate_logical_not(&tokens[i + 1..], ctx)?;
                    out.extend(operand_ts);
                    i += 1 + consumed;
                    prev_was_value = true;
                    continue;
                }
                "(" => {
                    // Try keyword cast first: (int)(x), (unsigned long)(x)
                    if let Some((cast_ts, consumed)) =
                        try_parse_cast(&tokens[i..], ctx)?
                    {
                        out.extend(cast_ts);
                        i += consumed;
                        prev_was_value = true;
                        continue;
                    }
                    let close = find_matching_paren(tokens, i)
                        .ok_or(SkipReason::Empty)?;
                    let inner_tokens = &tokens[i + 1..close];

                    // (ident)( — either a typedef cast like (my_t)(x),
                    // a parenthesized function call like (foo)(x), or
                    // a call-through-param like (f)(x). None are
                    // expressible in const fn: typedef casts produce
                    // invalid Rust, and function/param calls aren't
                    // allowed in const context (i64 isn't callable).
                    if inner_tokens.len() == 1 &&
                        inner_tokens[0].kind == CXToken_Identifier &&
                        tokens.get(close + 1).is_some_and(|t| {
                            t.kind == CXToken_Punctuation &&
                                t.spelling() == b"("
                        })
                    {
                        let name =
                            std::str::from_utf8(inner_tokens[0].spelling())
                                .unwrap_or("?");
                        return Err(SkipReason::FunctionCallOrCast(
                            name.to_owned(),
                        ));
                    }

                    let inner = translate_expr(inner_tokens, ctx)?;
                    out.extend(std::iter::once(proc_macro2::TokenTree::Group(
                        Group::new(Delimiter::Parenthesis, inner),
                    )));
                    i = close + 1;
                    prev_was_value = true;
                    continue;
                }
                _ => {
                    let op: TokenStream =
                        spelling.parse().map_err(|_| SkipReason::Empty)?;
                    out.extend(op);
                    prev_was_value = false;
                }
            },

            CXToken_Keyword => match spelling {
                "void" | "struct" | "union" | "enum" | "typeof" | "int" |
                "unsigned" | "signed" | "long" | "short" | "char" |
                "float" | "double" | "__asm__" | "__asm" | "asm" => {
                    return Err(SkipReason::TypeKeywordInBody(
                        spelling.to_owned(),
                    ));
                }
                "sizeof" => {
                    let (ts, consumed) =
                        translate_sizeof(&tokens[i + 1..], ctx)?;
                    out.extend(ts);
                    prev_was_value = true;
                    i += 1 + consumed;
                    continue;
                }
                _ => {
                    // Unknown keyword — emit as identifier. The syn
                    // validation will catch invalid constructs.
                    let ident = Ident::new(spelling, Span::call_site());
                    out.extend(quote! { #ident });
                }
            },

            CXToken_Comment => {}
            _ => {}
        }

        i += 1;
    }

    Ok(out)
}

// ---------------------------------------------------------------------------
// sizeof translation
// ---------------------------------------------------------------------------

/// Scan body tokens for `sizeof(param_name)` patterns and return the set of
/// parameter names used as `sizeof` arguments (these become generic type
/// params).
fn find_sizeof_params(
    tokens: &[OwnedToken],
    params: &[String],
    sizeof_fns: &[String],
    prior_macros: &[PriorFunctionMacro],
) -> Vec<String> {
    let mut sizeof_params = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        let spelling = std::str::from_utf8(tokens[i].spelling()).unwrap_or("");

        // Direct sizeof(param).
        let is_sizeof = (tokens[i].kind == CXToken_Identifier ||
            tokens[i].kind == CXToken_Keyword) &&
            spelling == "sizeof";

        // Call to a sizeof-like function: FUNC(param) where FUNC
        // takes only type params (e.g., _IOC_TYPECHECK).
        let is_sizeof_fn = tokens[i].kind == CXToken_Identifier &&
            sizeof_fns.iter().any(|f| f == spelling);

        if is_sizeof || is_sizeof_fn {
            if let Some(arg) = extract_sizeof_arg(&tokens[i + 1..], params) {
                if !sizeof_params.contains(&arg) {
                    sizeof_params.push(arg);
                }
            }
        }

        // Call to a known macro with mixed type/value params:
        // CALLEE(arg1, arg2, ...). If any arg position corresponds
        // to a callee type param AND the arg is one of our params,
        // that param becomes a type param.
        if tokens[i].kind == CXToken_Identifier {
            if let Some(callee) =
                prior_macros.iter().find(|m| m.name == spelling)
            {
                if !callee.type_param_names.is_empty() {
                    if let Some(args) = extract_call_args(&tokens[i + 1..]) {
                        for (pos, arg) in args.iter().enumerate() {
                            if let Some(param_name) =
                                callee.all_param_names.get(pos)
                            {
                                if callee.type_param_names.contains(param_name) &&
                                    params.contains(arg) &&
                                    !sizeof_params.contains(arg)
                                {
                                    sizeof_params.push(arg.clone());
                                }
                            }
                        }
                    }
                }
            }
        }

        i += 1;
    }
    sizeof_params
}

/// Extract single-identifier arguments from a `(arg1, arg2, ...)` call.
/// Returns a list of arg names if ALL args are single identifiers that
/// appear in `params`. Returns None if the call can't be parsed or has
/// complex args.
fn extract_call_args(tokens: &[OwnedToken]) -> Option<Vec<String>> {
    if tokens.is_empty() ||
        tokens[0].kind != CXToken_Punctuation ||
        tokens[0].spelling() != b"("
    {
        return None;
    }
    let close = find_matching_paren(tokens, 0)?;
    let arg_tokens = &tokens[1..close];
    let ranges = split_arg_ranges(arg_tokens);
    let mut args = Vec::new();
    for &(start, end) in &ranges {
        let arg = &arg_tokens[start..end];
        if arg.len() == 1 && arg[0].kind == CXToken_Identifier {
            let name = std::str::from_utf8(arg[0].spelling()).ok()?.to_owned();
            args.push(name);
        } else {
            // Complex arg — can't determine type param propagation.
            args.push(String::new());
        }
    }
    Some(args)
}

/// Try to translate a function call using callee metadata to decide
/// whether turbofish syntax is needed.
///
/// Looks up the callee in `ctx.prior_macros`. If the callee has type params,
/// the call arguments are matched positionally: callee type-param positions
/// become generic args, callee value-param positions become regular args.
///
/// If a caller passes a type param where the callee expects a value, the
/// caller macro is unskippable (returns `Err`).
///
/// `fn_name` is the raw callee name, `tokens` starts at `(`.
fn try_translate_call(
    fn_name: &str,
    tokens: &[OwnedToken],
    ctx: &TranslateCtx,
) -> Result<Option<(TokenStream, usize)>, SkipReason> {
    if tokens.is_empty() ||
        tokens[0].kind != CXToken_Punctuation ||
        tokens[0].spelling() != b"("
    {
        return Ok(None);
    }

    // Look up callee in prior macros.
    let callee = ctx.prior_macros.iter().find(|m| m.name == fn_name);
    let Some(callee) = callee else {
        // Unknown callee — forward reference. Return error so the
        // fixpoint pass defers this macro to a later wave.
        return Err(SkipReason::UnknownCallee(fn_name.to_owned()));
    };

    let Some(close) = find_matching_paren(tokens, 0) else {
        return Ok(None);
    };
    let arg_tokens = &tokens[1..close];
    let arg_ranges = split_arg_ranges(arg_tokens);
    let total_callee_params =
        callee.type_param_names.len() + callee.value_param_count;

    // If arg count doesn't match callee param count, the generated
    // call would fail with E0061. Skip the macro rather than emit
    // uncompilable code.
    if arg_ranges.len() != total_callee_params {
        return Err(SkipReason::ArityMismatch(format!(
            "`{fn_name}` expects {} args, got {}",
            total_callee_params,
            arg_ranges.len(),
        )));
    }

    // No type params on callee → regular call, no turbofish needed.
    if callee.type_param_names.is_empty() {
        // But check: is the caller passing a type param as a value arg?
        for &(start, end) in &arg_ranges {
            let arg = &arg_tokens[start..end];
            if arg.len() == 1 && arg[0].kind == CXToken_Identifier {
                let name = std::str::from_utf8(arg[0].spelling()).unwrap_or("");
                let escaped = escape_keyword(name);
                if ctx.type_params.contains(&escaped) {
                    // Passing a type param to a value-only callee — can't
                    // express this in const fn.
                    return Err(SkipReason::FunctionCallOrCast(format!(
                        "type param `{name}` passed as value to `{fn_name}`"
                    )));
                }
            }
        }
        return Ok(None);
    }

    // Callee has type params — build turbofish.
    // Use POSITIONAL matching: check whether the callee's parameter at
    // each position is a type param, rather than comparing names (which
    // breaks when caller/callee use different param names, e.g.,
    // _IOR(type,nr,argtype) calling _IOC_TYPECHECK(t)).
    let mut type_args = Vec::new();
    let mut value_args = Vec::new();

    for (arg_idx, &(start, end)) in arg_ranges.iter().enumerate() {
        let arg = &arg_tokens[start..end];

        // Is this position a type-param position in the callee?
        let callee_param_is_type = callee
            .all_param_names
            .get(arg_idx)
            .is_some_and(|p| callee.type_param_names.contains(p));

        if arg.len() == 1 && arg[0].kind == CXToken_Identifier {
            let name = std::str::from_utf8(arg[0].spelling()).unwrap_or("");
            let escaped = escape_keyword(name);

            if ctx.type_params.contains(&escaped) {
                if callee_param_is_type {
                    // Caller type param → callee type param: turbofish.
                    type_args.push(make_ident(name));
                    continue;
                }
                // Caller passes a type param but callee expects a value.
                return Err(SkipReason::FunctionCallOrCast(format!(
                    "type param `{name}` passed as value to `{fn_name}`"
                )));
            }

            if callee_param_is_type {
                // Callee expects type at this position, caller passes a
                // non-type-param identifier — treat as concrete type.
                type_args.push(make_ident(name));
                continue;
            }
        } else if callee_param_is_type {
            // Complex expression in a type param position — can't
            // express as turbofish generic argument.
            return Err(SkipReason::FunctionCallOrCast(format!(
                "complex expression in type param position for \
                 `{fn_name}`"
            )));
        }

        // Value argument.
        value_args.push(translate_expr(arg, ctx)?);
    }

    let fn_ident = make_ident(fn_name);
    let vt = ctx.vt();
    let ts = if value_args.is_empty() {
        quote! { (#fn_ident ::< #(#type_args),* >() as #vt) }
    } else {
        quote! { (#fn_ident ::< #(#type_args),* >( #(#value_args),* ) as #vt) }
    };

    Ok(Some((ts, close + 1)))
}

/// Split tokens into argument ranges by top-level commas.
/// Returns `(start, end)` index pairs into the token slice.
/// An empty token slice (from `()`) returns an empty vec (0 arguments).
fn split_arg_ranges(tokens: &[OwnedToken]) -> Vec<(usize, usize)> {
    if tokens.is_empty() {
        return Vec::new();
    }

    let mut ranges = Vec::new();
    let mut start = 0;
    let mut depth = 0i32;

    for (i, t) in tokens.iter().enumerate() {
        if t.kind == CXToken_Punctuation {
            match t.spelling() {
                b"(" => depth += 1,
                b")" => depth -= 1,
                b"," if depth == 0 => {
                    ranges.push((start, i));
                    start = i + 1;
                }
                _ => {}
            }
        }
    }
    if start <= tokens.len() {
        ranges.push((start, tokens.len()));
    }
    ranges
}

/// Extract the parameter name from a `sizeof(param)` expression.
///
/// `tokens` starts after the `sizeof` keyword.
fn extract_sizeof_arg(
    tokens: &[OwnedToken],
    params: &[String],
) -> Option<String> {
    if tokens.is_empty() {
        return None;
    }
    if tokens[0].kind == CXToken_Punctuation && tokens[0].spelling() == b"(" {
        let close = find_matching_paren(tokens, 0)?;
        let inner = &tokens[1..close];
        if inner.len() == 1 && inner[0].kind == CXToken_Identifier {
            let name =
                std::str::from_utf8(inner[0].spelling()).ok()?.to_owned();
            if params.contains(&name) {
                return Some(name);
            }
        }
    }
    None
}

/// Translate a `sizeof(...)` expression to `core::mem::size_of`.
///
/// Returns `(TokenStream, tokens_consumed)`.
fn translate_sizeof(
    tokens: &[OwnedToken],
    ctx: &TranslateCtx,
) -> Result<(TokenStream, usize), SkipReason> {
    if tokens.is_empty() {
        return Err(SkipReason::UntranslatableSizeof);
    }

    if tokens[0].kind == CXToken_Punctuation && tokens[0].spelling() == b"(" {
        let close = find_matching_paren(tokens, 0)
            .ok_or(SkipReason::UntranslatableSizeof)?;
        let inner = &tokens[1..close];

        // Type parameter: sizeof(T) -> core::mem::size_of::<T>() as vt
        if inner.len() == 1 && inner[0].kind == CXToken_Identifier {
            let name = std::str::from_utf8(inner[0].spelling())
                .map_err(|_| SkipReason::UntranslatableSizeof)?;
            let escaped = escape_keyword(name);
            if ctx.type_params.contains(&escaped) {
                let type_ident = make_ident(name);
                let vt = ctx.vt();
                return Ok((
                    quote! { core::mem::size_of::<#type_ident>() as #vt },
                    close + 1,
                ));
            }
        }

        // Concrete C type: sizeof(int) -> core::mem::size_of::<i32>() as vt
        if let Some(rust_type) =
            try_translate_c_type(inner, ctx.target_pointer_size)
        {
            let rt = parse_ts(&rust_type);
            let vt = ctx.vt();
            return Ok((
                quote! { core::mem::size_of::<#rt>() as #vt },
                close + 1,
            ));
        }
    }

    Err(SkipReason::UntranslatableSizeof)
}

// ---------------------------------------------------------------------------
// Logical NOT translation
// ---------------------------------------------------------------------------

/// Build a logical NOT expression: `(((operand) == 0) as vt)` for integers,
/// `((operand == 0.0) as vt)` for floats.
fn logical_not_expr(operand: &TokenStream, ctx: &TranslateCtx) -> TokenStream {
    let vt = ctx.vt();
    if matches!(ctx.value_type, "f64" | "f32") {
        // Float truthiness: 0.0 is false, anything else is true.
        // Can't cast bool directly to f64 — go through i64.
        quote! { (((#operand) == 0.0) as i64 as #vt) }
    } else {
        // Integer truthiness via cast to i64.
        quote! { (((#operand) as i64 == 0) as #vt) }
    }
}

/// Translate C's logical NOT `!operand` to Rust.
///
/// C's `!` is logical NOT: `!0` = 1, `!nonzero` = 0.
/// Rust's `!` on integers is bitwise NOT (completely different).
///
/// For integer types: `!operand` → `(((operand) as i64 == 0) as vt)`
/// For float types:   `!operand` → `(((operand) == 0.0) as vt)`
///
/// `tokens` starts after the `!`. Returns `(TokenStream, tokens_consumed)`.
fn translate_logical_not(
    tokens: &[OwnedToken],
    ctx: &TranslateCtx,
) -> Result<(TokenStream, usize), SkipReason> {
    if tokens.is_empty() {
        return Err(SkipReason::Empty);
    }

    // Parenthesized operand: !(expr)
    if tokens[0].kind == CXToken_Punctuation && tokens[0].spelling() == b"(" {
        let close = find_matching_paren(tokens, 0).ok_or(SkipReason::Empty)?;
        let inner = translate_expr(&tokens[1..close], ctx)?;
        return Ok((logical_not_expr(&inner, ctx), close + 1));
    }

    let operand_spelling = std::str::from_utf8(tokens[0].spelling())
        .map_err(|_| SkipReason::Empty)?;

    // Chained NOT: !!x → recurse.
    if tokens[0].kind == CXToken_Punctuation && operand_spelling == "!" {
        let (inner, consumed) = translate_logical_not(&tokens[1..], ctx)?;
        return Ok((logical_not_expr(&inner, ctx), 1 + consumed));
    }

    // sizeof operand: !sizeof(T) → NOT of size_of.
    if (tokens[0].kind == CXToken_Identifier ||
        tokens[0].kind == CXToken_Keyword) &&
        operand_spelling == "sizeof"
    {
        let (sizeof_ts, consumed) = translate_sizeof(&tokens[1..], ctx)?;
        return Ok((logical_not_expr(&sizeof_ts, ctx), 1 + consumed));
    }

    // Identifier operand — but check if it's a function call: !ID(x).
    if tokens[0].kind == CXToken_Identifier {
        // Reject non-const variables (same check as translate_tokens).
        if ctx.non_const_vars.contains(operand_spelling) {
            return Err(SkipReason::MutableStaticReference(
                operand_spelling.to_owned(),
            ));
        }

        if tokens.get(1).is_some_and(|t| {
            t.kind == CXToken_Punctuation && t.spelling() == b"("
        }) {
            let close =
                find_matching_paren(tokens, 1).ok_or(SkipReason::Empty)?;
            let call_tokens = &tokens[..=close];
            let call_ts = translate_tokens(call_tokens, ctx)?;
            return Ok((logical_not_expr(&call_ts, ctx), close + 1));
        }

        let ident = make_ident(operand_spelling);
        return Ok((logical_not_expr(&quote! { #ident }, ctx), 1));
    }

    // Literal operand — use translate_literal for C suffix stripping.
    if tokens[0].kind == CXToken_Literal {
        let lit_ts = translate_literal(operand_spelling, ctx)?;
        return Ok((logical_not_expr(&lit_ts, ctx), 1));
    }

    // Other single-token operand (unlikely but safe fallback).
    let operand: TokenStream =
        operand_spelling.parse().map_err(|_| SkipReason::Empty)?;
    Ok((logical_not_expr(&operand, ctx), 1))
}

// ---------------------------------------------------------------------------
// C cast detection and translation
// ---------------------------------------------------------------------------

/// Try to parse a C-style cast at the current position.
///
/// Detects `(type_keywords)expr` and returns the translated Rust `as`
/// expression with the number of tokens consumed.
fn try_parse_cast(
    tokens: &[OwnedToken],
    ctx: &TranslateCtx,
) -> Result<Option<(TokenStream, usize)>, SkipReason> {
    if tokens.is_empty() ||
        tokens[0].kind != CXToken_Punctuation ||
        tokens[0].spelling() != b"("
    {
        return Ok(None);
    }

    let Some(close) = find_matching_paren(tokens, 0) else {
        return Ok(None);
    };
    let inner = &tokens[1..close];

    if inner.is_empty() {
        return Ok(None);
    }

    let all_type_keywords = inner.iter().all(|t| {
        t.kind == CXToken_Keyword &&
            matches!(
                std::str::from_utf8(t.spelling()).unwrap_or(""),
                "int" |
                    "unsigned" |
                    "signed" |
                    "long" |
                    "short" |
                    "char" |
                    "float" |
                    "double"
            )
    });

    if !all_type_keywords {
        return Ok(None);
    }

    let Some(rust_type) = try_translate_c_type(inner, ctx.target_pointer_size)
    else {
        return Ok(None);
    };

    let rest = &tokens[close + 1..];
    if rest.is_empty() {
        return Ok(None);
    }

    let rt = parse_ts(&rust_type);
    let vt = ctx.vt();

    if rest[0].kind == CXToken_Punctuation && rest[0].spelling() == b"(" {
        // (type)(expr) — parenthesized operand.
        let expr_close =
            find_matching_paren(rest, 0).ok_or(SkipReason::Empty)?;
        let expr = translate_expr(&rest[1..expr_close], ctx)?;
        let consumed = close + 1 + expr_close + 1;
        Ok(Some((quote! { ((#expr) as #rt as #vt) }, consumed)))
    } else if rest[0].kind == CXToken_Identifier ||
        rest[0].kind == CXToken_Literal
    {
        // (type)operand — single token operand (identifier or literal).
        let operand = std::str::from_utf8(rest[0].spelling())
            .map_err(|_| SkipReason::Empty)?;
        // Reject non-const variables (same check as translate_tokens).
        if rest[0].kind == CXToken_Identifier &&
            ctx.non_const_vars.contains(operand)
        {
            return Err(SkipReason::MutableStaticReference(operand.to_owned()));
        }
        let operand_ts = translate_cast_operand(operand);
        let consumed = close + 1 + 1;
        Ok(Some((quote! { ((#operand_ts) as #rt as #vt) }, consumed)))
    } else if rest[0].kind == CXToken_Punctuation &&
        (rest[0].spelling() == b"-" || rest[0].spelling() == b"+") &&
        rest.len() > 1
    {
        // (type)-operand or (type)+operand — unary prefix after cast.
        let prefix: TokenStream = std::str::from_utf8(rest[0].spelling())
            .map_err(|_| SkipReason::Empty)?
            .parse()
            .map_err(|_| SkipReason::Empty)?;

        if rest[1].kind == CXToken_Punctuation && rest[1].spelling() == b"(" {
            // (type)-(expr) — prefix + parenthesized expression.
            let expr_close =
                find_matching_paren(rest, 1).ok_or(SkipReason::Empty)?;
            let inner = translate_expr(&rest[2..expr_close], ctx)?;
            let consumed = close + 1 + expr_close + 1;
            Ok(Some((
                quote! { ((#prefix (#inner)) as #rt as #vt) },
                consumed,
            )))
        } else if rest[1].kind == CXToken_Identifier ||
            rest[1].kind == CXToken_Literal
        {
            // (type)-token — prefix + single identifier or literal.
            let operand = std::str::from_utf8(rest[1].spelling())
                .map_err(|_| SkipReason::Empty)?;
            if rest[1].kind == CXToken_Identifier &&
                ctx.non_const_vars.contains(operand)
            {
                return Err(SkipReason::MutableStaticReference(
                    operand.to_owned(),
                ));
            }
            let operand_ts = translate_cast_operand(operand);
            let consumed = close + 1 + 2;
            Ok(Some((
                quote! { ((#prefix #operand_ts) as #rt as #vt) },
                consumed,
            )))
        } else {
            Ok(None)
        }
    } else {
        // Operand we can't handle. Fall back to normal paren
        // processing which will reject the type keywords.
        Ok(None)
    }
}

/// Translate a single-token cast operand. Handles both identifiers (including
/// keywords) and numeric/char literals.
fn translate_cast_operand(operand: &str) -> TokenStream {
    // Try as a number literal first.
    if operand.starts_with(|c: char| c.is_ascii_digit()) {
        let num = strip_c_suffix(operand);
        let num_str =
            try_convert_c_octal(num).unwrap_or_else(|| num.to_owned());
        return num_str.parse().unwrap_or_default();
    }
    // Character literal.
    if operand.starts_with('\'') {
        return operand.parse().unwrap_or_default();
    }
    // Identifier (possibly a keyword).
    let ident = make_ident(operand);
    quote! { #ident }
}

/// Translate a sequence of C type keyword tokens to a Rust type name.
///
/// `target_pointer_size` is in bytes (4 for 32-bit, 8 for 64-bit) and
/// controls the mapping of `long` / `unsigned long` which are
/// pointer-width on most platforms.
fn try_translate_c_type(
    tokens: &[OwnedToken],
    target_pointer_size: usize,
) -> Option<String> {
    let words: Vec<&str> = tokens
        .iter()
        .filter_map(|t| std::str::from_utf8(t.spelling()).ok())
        .collect();

    let type_str = words.join(" ");

    // long/unsigned long are target-dependent: 32-bit on ILP32, 64-bit
    // on LP64. long long is always 64-bit.
    let (long_signed, long_unsigned) = if target_pointer_size >= 8 {
        ("i64", "u64")
    } else {
        ("i32", "u32")
    };

    match type_str.as_str() {
        "unsigned long long" => Some("u64".to_owned()),
        "long long" | "signed long long" => Some("i64".to_owned()),
        "unsigned long" => Some(long_unsigned.to_owned()),
        "long" | "signed long" => Some(long_signed.to_owned()),
        "unsigned int" | "unsigned" => Some("u32".to_owned()),
        "int" | "signed int" | "signed" => Some("i32".to_owned()),
        "unsigned short" => Some("u16".to_owned()),
        "short" | "signed short" => Some("i16".to_owned()),
        "unsigned char" => Some("u8".to_owned()),
        "char" | "signed char" => Some("i8".to_owned()),
        "float" => Some("f32".to_owned()),
        "double" => Some("f64".to_owned()),
        _ if words.len() == 2 &&
            matches!(words[0], "struct" | "union" | "enum") =>
        {
            Some(words[1].to_owned())
        }
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Literal translation
// ---------------------------------------------------------------------------

/// Translate a C literal to a `TokenStream`.
fn translate_literal(
    lit: &str,
    ctx: &TranslateCtx,
) -> Result<TokenStream, SkipReason> {
    // String literals can't match integer/float return types.
    if lit.starts_with('"') {
        return Err(SkipReason::InvalidExpression);
    }
    // Character literal — cast to value type.
    if lit.starts_with('\'') {
        let char_ts: TokenStream = lit.parse().unwrap_or_default();
        let vt = ctx.vt();
        return Ok(quote! { (#char_ts as #vt) });
    }
    // Number literal — strip C suffixes, parse as bare number.
    let num = strip_c_suffix(lit);
    // Float literals with an integer value type produce type mismatches.
    // If the value type is f64/f32, the literal is allowed through.
    if is_float_literal(num) && !matches!(ctx.value_type, "f64" | "f32") {
        return Err(SkipReason::InvalidExpression);
    }
    // Convert C octal (0644) to Rust octal (0o644).
    let num_str = try_convert_c_octal(num).unwrap_or_else(|| num.to_owned());
    let num_ts: TokenStream = num_str.parse().unwrap_or_default();
    // When value type is float but the literal is integer (e.g., `0`
    // in a macro that also has `1.0f`), cast so Rust doesn't infer
    // it as an integer in comparison/arithmetic context.
    if matches!(ctx.value_type, "f64" | "f32") && !is_float_literal(num) {
        let vt = ctx.vt();
        return Ok(quote! { (#num_ts as #vt) });
    }
    Ok(num_ts)
}

/// Convert a C octal literal to Rust syntax.
///
/// C uses `0644` for octal; Rust uses `0o644`. Returns `None` if not octal.
/// A literal is C octal if it starts with `0`, has more than one digit, and
/// doesn't have a `0x`/`0X`/`0b`/`0B` prefix.
fn try_convert_c_octal(num: &str) -> Option<String> {
    if num.len() > 1 &&
        num.starts_with('0') &&
        !num.starts_with("0x") &&
        !num.starts_with("0X") &&
        !num.starts_with("0b") &&
        !num.starts_with("0B") &&
        !num.starts_with("0o") &&
        !num.starts_with("0O") &&
        num[1..].chars().all(|c| c.is_ascii_digit())
    {
        Some(format!("0o{}", &num[1..]))
    } else {
        None
    }
}

/// Check if a (suffix-stripped) numeric literal exceeds `u32::MAX`.
fn literal_exceeds_u32(num: &str) -> bool {
    let val = if num.starts_with("0x") || num.starts_with("0X") {
        u64::from_str_radix(&num[2..], 16).ok()
    } else if num.len() > 1 &&
        num.starts_with('0') &&
        num[1..].chars().all(|c| c.is_ascii_digit())
    {
        // C octal (leading zero).
        u64::from_str_radix(&num[1..], 8).ok()
    } else {
        num.parse::<u64>().ok()
    };
    val.is_some_and(|v| v > u64::from(u32::MAX))
}

/// Check if a (suffix-stripped) numeric literal is a float.
/// Detects decimal points (`1.5`) and exponent notation (`1e3`, `1E-3`).
fn is_float_literal(num: &str) -> bool {
    // Must not be hex (0x prefix) — 'e'/'E' are hex digits there.
    if num.starts_with("0x") || num.starts_with("0X") {
        return false;
    }
    num.contains('.') || num.contains('e') || num.contains('E')
}

/// Strip C type suffixes (`U`, `L`, `UL`, `ULL`, `f`, `F`, etc.) from a
/// number literal. Also handles MSVC-specific suffixes (`i8`, `i16`,
/// `i32`, `i64`, `ui8`, `ui16`, `ui32`, `ui64`).
fn strip_c_suffix(lit: &str) -> &str {
    // MSVC-specific suffixes: i8, i16, i32, i64, ui8, ui16, ui32, ui64
    // in any case combination (Clang in MSVC mode accepts mixed case).
    // Check these first since they contain digits that would confuse
    // the simpler u/U/l/L scan below.
    {
        let lower = lit.to_ascii_lowercase();
        for suffix in
            &["ui64", "ui32", "ui16", "ui8", "i64", "i32", "i16", "i8"]
        {
            if lower.ends_with(suffix) {
                let stripped = &lit[..lit.len() - suffix.len()];
                if !stripped.is_empty() {
                    return stripped;
                }
            }
        }
    }

    // Strip standard C integer suffixes: u/U/l/L.
    let end = lit.find(['u', 'U', 'l', 'L']).unwrap_or(lit.len());
    let num = &lit[..end];
    let num = if num.is_empty() { lit } else { num };

    // Strip float suffix f/F from the end, but NOT for hex literals
    // (where f/F are valid hex digits, e.g., 0xFF).
    if !num.starts_with("0x") && !num.starts_with("0X") {
        if let Some(stripped) =
            num.strip_suffix('f').or_else(|| num.strip_suffix('F'))
        {
            if !stripped.is_empty() {
                return stripped;
            }
        }
    }

    num
}

// ---------------------------------------------------------------------------
// Post-syn semantic checks
// ---------------------------------------------------------------------------

/// Recursively check a parsed expression for constructs that are
/// syntactically valid Rust but produce type mismatches in our generated
/// `const fn` (which returns an integer type).
fn expr_has_unsupported_construct(expr: &syn::Expr) -> bool {
    match expr {
        // C comma operator becomes a Rust tuple — type mismatch.
        syn::Expr::Tuple(_) => true,
        // Recurse into common wrapper expressions.
        syn::Expr::Paren(p) => expr_has_unsupported_construct(&p.expr),
        syn::Expr::Group(g) => expr_has_unsupported_construct(&g.expr),
        syn::Expr::Binary(b) => {
            expr_has_unsupported_construct(&b.left) ||
                expr_has_unsupported_construct(&b.right)
        }
        syn::Expr::Unary(u) => expr_has_unsupported_construct(&u.expr),
        syn::Expr::Cast(c) => expr_has_unsupported_construct(&c.expr),
        syn::Expr::Call(c) => {
            c.args.iter().any(expr_has_unsupported_construct)
        }
        syn::Expr::If(i) => {
            expr_has_unsupported_construct(&i.cond) ||
                i.then_branch.stmts.iter().any(|s| {
                    matches!(s, syn::Stmt::Expr(e, _) if expr_has_unsupported_construct(e))
                }) ||
                i.else_branch
                    .as_ref()
                    .is_some_and(|(_, e)| {
                        expr_has_unsupported_construct(e)
                    })
        }
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Utility
// ---------------------------------------------------------------------------

/// Find the matching close paren for the open paren at `tokens[pos]`.
fn find_matching_paren(tokens: &[OwnedToken], pos: usize) -> Option<usize> {
    let mut depth = 0i32;
    for (i, t) in tokens[pos..].iter().enumerate() {
        if t.kind == CXToken_Punctuation {
            match t.spelling() {
                b"(" => depth += 1,
                b")" => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(pos + i);
                    }
                }
                _ => {}
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_c_suffix() {
        assert_eq!(strip_c_suffix("42"), "42");
        assert_eq!(strip_c_suffix("42U"), "42");
        assert_eq!(strip_c_suffix("42UL"), "42");
        assert_eq!(strip_c_suffix("42ULL"), "42");
        assert_eq!(strip_c_suffix("42L"), "42");
        assert_eq!(strip_c_suffix("42LL"), "42");
        assert_eq!(strip_c_suffix("0xFF"), "0xFF");
        assert_eq!(strip_c_suffix("0xFFU"), "0xFF");
        assert_eq!(strip_c_suffix("0xFFUL"), "0xFF");
        // Float suffix f/F.
        assert_eq!(strip_c_suffix("1.5f"), "1.5");
        assert_eq!(strip_c_suffix("1.5F"), "1.5");
        assert_eq!(strip_c_suffix("2.0f"), "2.0");
        // Hex literals: f/F are hex digits, NOT suffixes.
        assert_eq!(strip_c_suffix("0xFf"), "0xFf");
        assert_eq!(strip_c_suffix("0xABCDEF"), "0xABCDEF");
        // MSVC suffixes.
        assert_eq!(strip_c_suffix("42i64"), "42");
        assert_eq!(strip_c_suffix("42i32"), "42");
        assert_eq!(strip_c_suffix("42ui64"), "42");
        assert_eq!(strip_c_suffix("42ui32"), "42");
        assert_eq!(strip_c_suffix("42i8"), "42");
        assert_eq!(strip_c_suffix("0xFFui64"), "0xFF");
        // MSVC suffixes: any case combination.
        assert_eq!(strip_c_suffix("42I64"), "42");
        assert_eq!(strip_c_suffix("42UI64"), "42");
        assert_eq!(strip_c_suffix("42I32"), "42");
        assert_eq!(strip_c_suffix("42Ui64"), "42");
        assert_eq!(strip_c_suffix("42uI64"), "42");
    }

    #[test]
    fn test_try_convert_c_octal() {
        // C octal → Rust octal.
        assert_eq!(try_convert_c_octal("0644"), Some("0o644".to_owned()));
        assert_eq!(try_convert_c_octal("077"), Some("0o77".to_owned()));
        assert_eq!(try_convert_c_octal("00"), Some("0o0".to_owned()));
        // NOT octal: hex, binary, single zero, decimal.
        assert_eq!(try_convert_c_octal("0xFF"), None);
        assert_eq!(try_convert_c_octal("0b101"), None);
        assert_eq!(try_convert_c_octal("0"), None);
        assert_eq!(try_convert_c_octal("42"), None);
        // Already Rust octal.
        assert_eq!(try_convert_c_octal("0o77"), None);
    }

    #[test]
    fn test_literal_exceeds_u32() {
        // Fits in u32.
        assert!(!literal_exceeds_u32("0"));
        assert!(!literal_exceeds_u32("1"));
        assert!(!literal_exceeds_u32("4294967295")); // u32::MAX
        assert!(!literal_exceeds_u32("0xFFFFFFFF"));
        assert!(!literal_exceeds_u32("037777777777")); // octal u32::MAX
                                                       // Exceeds u32.
        assert!(literal_exceeds_u32("4294967296")); // u32::MAX + 1
        assert!(literal_exceeds_u32("0x100000000"));
        assert!(literal_exceeds_u32("040000000000")); // octal u32::MAX + 1
        assert!(literal_exceeds_u32("0xFFFFFFFFFFFFFFFF"));
    }

    #[test]
    fn test_make_ident_keyword() {
        let ident = make_ident("type");
        assert_eq!(ident.to_string(), "r#type");
    }

    #[test]
    fn test_make_ident_normal() {
        let ident = make_ident("foo");
        assert_eq!(ident.to_string(), "foo");
    }

    #[test]
    fn test_skip_reason_display() {
        assert_eq!(SkipReason::Variadic.to_string(), "variadic macro");
        assert_eq!(
            SkipReason::TypeKeywordInBody("void".into()).to_string(),
            "untranslatable C keyword `void` in body"
        );
    }

    #[test]
    fn test_infer_value_type_empty() {
        let types = HashMap::new();
        assert_eq!(infer_value_type(&[], &[], &types), "i64");
    }

    #[test]
    fn test_translate_cast_operand_literal() {
        let ts = translate_cast_operand("42");
        assert_eq!(ts.to_string(), "42");
    }

    #[test]
    fn test_translate_cast_operand_ident() {
        let ts = translate_cast_operand("foo");
        assert_eq!(ts.to_string(), "foo");
    }
}
