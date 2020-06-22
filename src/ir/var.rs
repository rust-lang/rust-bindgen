//! Intermediate representation of variables.

use super::super::codegen::MacroTypeVariation;
use super::context::{BindgenContext, TypeId};
use super::dot::DotAttributes;
use super::function::cursor_mangling;
use super::int::IntKind;
use super::item::Item;
use super::ty::{FloatKind, TypeKind};
use crate::callbacks::MacroParsingBehavior;
use crate::clang;
use crate::clang::ClangToken;
use crate::parse::{
    ClangItemParser, ClangSubItemParser, ParseError, ParseResult,
};
use saltwater::{InternedStr, Literal};
use std::collections::HashMap;
use std::io;

/// The type for a constant variable.
#[derive(Debug)]
pub enum VarType {
    /// A boolean.
    Bool(bool),
    /// An integer.
    Int(i64),
    /// A floating point number.
    Float(f64),
    /// A character.
    Char(u8),
    /// A string, not necessarily well-formed utf-8.
    String(Vec<u8>),
}

/// A `Var` is our intermediate representation of a variable.
#[derive(Debug)]
pub struct Var {
    /// The name of the variable.
    name: String,
    /// The mangled name of the variable.
    mangled_name: Option<String>,
    /// The type of the variable.
    ty: TypeId,
    /// The value of the variable, that needs to be suitable for `ty`.
    val: Option<VarType>,
    /// Whether this variable is const.
    is_const: bool,
}

impl Var {
    /// Construct a new `Var`.
    pub fn new(
        name: String,
        mangled_name: Option<String>,
        ty: TypeId,
        val: Option<VarType>,
        is_const: bool,
    ) -> Var {
        assert!(!name.is_empty());
        Var {
            name,
            mangled_name,
            ty,
            val,
            is_const,
        }
    }

    /// Is this variable `const` qualified?
    pub fn is_const(&self) -> bool {
        self.is_const
    }

    /// The value of this constant variable, if any.
    pub fn val(&self) -> Option<&VarType> {
        self.val.as_ref()
    }

    /// Get this variable's type.
    pub fn ty(&self) -> TypeId {
        self.ty
    }

    /// Get this variable's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get this variable's mangled name.
    pub fn mangled_name(&self) -> Option<&str> {
        self.mangled_name.as_ref().map(|n| &**n)
    }
}

impl DotAttributes for Var {
    fn dot_attributes<W>(
        &self,
        _ctx: &BindgenContext,
        out: &mut W,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        if self.is_const {
            writeln!(out, "<tr><td>const</td><td>true</td></tr>")?;
        }

        if let Some(ref mangled) = self.mangled_name {
            writeln!(
                out,
                "<tr><td>mangled name</td><td>{}</td></tr>",
                mangled
            )?;
        }

        Ok(())
    }
}

// TODO(emilio): we could make this more (or less) granular, I guess.
fn default_macro_constant_type(ctx: &BindgenContext, value: i64) -> IntKind {
    if value < 0 ||
        ctx.options().default_macro_constant_type ==
            MacroTypeVariation::Signed
    {
        if value < i32::min_value() as i64 || value > i32::max_value() as i64 {
            IntKind::I64
        } else {
            IntKind::I32
        }
    } else if value > u32::max_value() as i64 {
        IntKind::U64
    } else {
        IntKind::U32
    }
}

/// Determines whether a set of tokens from a CXCursor_MacroDefinition
/// represent a function-like macro. If so, calls the func_macro callback
/// and returns `Err(ParseError::Continue)` to signal to skip further
/// processing. If conversion to UTF-8 fails (it is performed only where it
/// should be infallible), then `Err(ParseError::Continue)` is returned as well.
fn handle_function_macro(
    cursor: &clang::Cursor,
    tokens: &[ClangToken],
    callbacks: &dyn crate::callbacks::ParseCallbacks,
) -> Result<(), ParseError> {
    // TODO: Hoist the `is_macro_function_like` check into this function's
    // caller, and thus avoid allocating the `tokens` vector for non-functional
    // macros.
    let is_functional_macro = cursor.is_macro_function_like();

    if !is_functional_macro {
        return Ok(());
    }

    let is_closing_paren = |t: &ClangToken| {
        // Test cheap token kind before comparing exact spellings.
        t.kind == clang_sys::CXToken_Punctuation && t.spelling() == b")"
    };
    let boundary = tokens.iter().position(is_closing_paren);

    let mut spelled = tokens.iter().map(ClangToken::spelling);
    // Add 1, to convert index to length.
    let left = spelled
        .by_ref()
        .take(boundary.ok_or(ParseError::Continue)? + 1);
    let left = left.collect::<Vec<_>>().concat();
    let left = String::from_utf8(left).map_err(|_| ParseError::Continue)?;
    let right = spelled;
    // Drop last token with LLVM < 4.0, due to an LLVM bug.
    //
    // See:
    //   https://bugs.llvm.org//show_bug.cgi?id=9069
    let len = match (right.len(), crate::clang_version().parsed) {
        (len, Some((v, _))) if len > 0 && v < 4 => len - 1,
        (len, _) => len,
    };
    let right: Vec<_> = right.take(len).collect();
    callbacks.func_macro(&left, &right);

    // We handled the macro, skip future macro processing.
    Err(ParseError::Continue)
}

impl ClangSubItemParser for Var {
    fn parse(
        cursor: clang::Cursor,
        ctx: &mut BindgenContext,
    ) -> Result<ParseResult<Self>, ParseError> {
        use clang_sys::*;
        match cursor.kind() {
            CXCursor_MacroDefinition => {
                let tokens: Vec<_> = cursor.tokens().iter().collect();

                if let Some(callbacks) = ctx.parse_callbacks() {
                    match callbacks.will_parse_macro(&cursor.spelling()) {
                        MacroParsingBehavior::Ignore => {
                            return Err(ParseError::Continue);
                        }
                        MacroParsingBehavior::Default => {}
                    }

                    handle_function_macro(&cursor, &tokens, callbacks)?;
                }

                let value = parse_macro(ctx, &tokens);

                let (id, value) = match value {
                    Some(v) => v,
                    None => return Err(ParseError::Continue),
                };

                assert!(!id.is_empty(), "Empty macro name?");

                let previously_defined = ctx.parsed_macro(id);

                // NB: It's important to "note" the macro even if the result is
                // not an integer, otherwise we might lose other kind of
                // derived macros.
                ctx.note_parsed_macro(id, value.clone());

                if previously_defined {
                    warn!("Duplicated macro definition: {}", id);
                    return Err(ParseError::Continue);
                }

                let parse_int = |value| {
                    let kind = ctx
                        .parse_callbacks()
                        .and_then(|c| {
                            c.int_macro(saltwater::get_str!(id), value)
                        })
                        .unwrap_or_else(|| default_macro_constant_type(&ctx, value));

                    (TypeKind::Int(kind), VarType::Int(value))
                };

                let (type_kind, val) = match value {
                    Literal::Float(f) => {
                        (TypeKind::Float(FloatKind::Double), VarType::Float(f))
                    }
                    Literal::Char(c) => {
                        (TypeKind::Int(IntKind::U8), VarType::Char(c))
                    }
                    Literal::Str(val) => {
                        let char_ty = Item::builtin_type(
                            TypeKind::Int(IntKind::U8),
                            true,
                            ctx,
                        );
                        if let Some(callbacks) = ctx.parse_callbacks() {
                            callbacks.str_macro(saltwater::get_str!(id), &val);
                        }
                        (TypeKind::Pointer(char_ty), VarType::String(val))
                    }
                    Literal::Int(i) => parse_int(i),
                    Literal::UnsignedInt(u) => parse_int(u as i64),
                };

                let ty = Item::builtin_type(type_kind, true, ctx);

                Ok(ParseResult::New(
                    Var::new(id.resolve_and_clone(), None, ty, Some(val), true),
                    Some(cursor),
                ))
            }
            CXCursor_VarDecl => {
                let name = cursor.spelling();
                if name.is_empty() {
                    warn!("Empty constant name?");
                    return Err(ParseError::Continue);
                }

                let ty = cursor.cur_type();

                // TODO(emilio): do we have to special-case constant arrays in
                // some other places?
                let is_const = ty.is_const() ||
                    (ty.kind() == CXType_ConstantArray &&
                        ty.elem_type()
                            .map_or(false, |element| element.is_const()));

                let ty = match Item::from_ty(&ty, cursor, None, ctx) {
                    Ok(ty) => ty,
                    Err(e) => {
                        assert_eq!(
                            ty.kind(),
                            CXType_Auto,
                            "Couldn't resolve constant type, and it \
                             wasn't an nondeductible auto type!"
                        );
                        return Err(e);
                    }
                };

                // Note: Ty might not be totally resolved yet, see
                // tests/headers/inner_const.hpp
                //
                // That's fine because in that case we know it's not a literal.
                let canonical_ty = ctx
                    .safe_resolve_type(ty)
                    .and_then(|t| t.safe_canonical_type(ctx));

                let is_integer = canonical_ty.map_or(false, |t| t.is_integer());
                let is_float = canonical_ty.map_or(false, |t| t.is_float());

                // TODO: We could handle `char` more gracefully.
                // TODO: Strings, though the lookup is a bit more hard (we need
                // to look at the canonical type of the pointee too, and check
                // is char, u8, or i8 I guess).
                let value = if is_integer {
                    let kind = match *canonical_ty.unwrap().kind() {
                        TypeKind::Int(kind) => kind,
                        _ => unreachable!(),
                    };

                    let mut val = cursor.evaluate().and_then(|v| v.as_int());
                    if val.is_none() || !kind.signedness_matches(val.unwrap()) {
                        let tu = ctx.translation_unit();
                        val = get_integer_literal_from_cursor(&cursor, tu);
                    }

                    val.map(|val| {
                        if kind == IntKind::Bool {
                            VarType::Bool(val != 0)
                        } else {
                            VarType::Int(val)
                        }
                    })
                } else if is_float {
                    cursor
                        .evaluate()
                        .and_then(|v| v.as_double())
                        .map(VarType::Float)
                } else {
                    cursor
                        .evaluate()
                        .and_then(|v| v.as_literal_string())
                        .map(VarType::String)
                };

                let mangling = cursor_mangling(ctx, &cursor);
                let var = Var::new(name, mangling, ty, value, is_const);

                Ok(ParseResult::New(var, Some(cursor)))
            }
            _ => {
                /* TODO */
                Err(ParseError::Continue)
            }
        }
    }
}

/// Try and parse an object macro using all the macros parsed until now.
///
/// The cursor includes the `id` token but not the `#define`.
fn parse_macro(
    ctx: &BindgenContext,
    tokens: &[ClangToken],
) -> Option<(InternedStr, Literal)> {
    use saltwater::Token;

    let mut swcc_tokens = tokens.iter().filter_map(ClangToken::as_swcc_token);
    let ident_str = match swcc_tokens.next()?.data {
        Token::Id(id) => id,
        _ => return None,
    };
    if ident_str.is_empty() || tokens.len() < 2 {
        return None;
    }
    let parsed_macros = ctx.parsed_macros();

    swcc_expr(swcc_tokens.collect(), &parsed_macros)
        .map(|literal| (ident_str, literal))
}

fn swcc_expr(
    mut tokens: Vec<saltwater::Locatable<saltwater::Token>>,
    definitions: &HashMap<InternedStr, saltwater::Definition>,
) -> Option<Literal> {
    use saltwater::{Locatable, PreProcessor};

    let parse = |tokens: Vec<Locatable<_>>| {
        let mut tokens = tokens.into_iter().peekable();
        let location = tokens.peek()?.location;
        PreProcessor::cpp_expr(definitions, tokens, location)
            .ok()?
            .const_fold()
            .ok()?
            .into_literal()
            .ok()
    };

    // TODO: remove this clone (requires changes in saltwater)
    if let Some(literal) = parse(tokens.clone()) {
        return Some(literal);
    }

    // Try without the last token, to workaround a libclang bug in versions
    // previous to 4.0.
    //
    // See:
    //   https://bugs.llvm.org//show_bug.cgi?id=9069
    //   https://reviews.llvm.org/D26446
    tokens.pop();
    parse(tokens)
}

fn parse_int_literal_tokens(cursor: &clang::Cursor) -> Option<i64> {
    let swcc_tokens = cursor.swcc_tokens();

    // TODO(emilio): We can try to parse other kinds of literals.
    match swcc_expr(swcc_tokens, &HashMap::new()) {
        Some(Literal::Int(i)) => Some(i),
        Some(Literal::UnsignedInt(u)) => Some(u as i64),
        _ => None,
    }
}

fn get_integer_literal_from_cursor(
    cursor: &clang::Cursor,
    unit: &clang::TranslationUnit,
) -> Option<i64> {
    use clang_sys::*;
    let mut value = None;
    cursor.visit(|c| {
        match c.kind() {
            CXCursor_IntegerLiteral | CXCursor_UnaryOperator => {
                value = parse_int_literal_tokens(&c);
            }
            CXCursor_UnexposedExpr => {
                value = get_integer_literal_from_cursor(&c, unit);
            }
            _ => (),
        }
        if value.is_some() {
            CXChildVisit_Break
        } else {
            CXChildVisit_Continue
        }
    });
    value
}
