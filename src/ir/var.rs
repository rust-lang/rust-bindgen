//! Intermediate representation of variables.

use clang;
use parse::{ClangItemParser, ClangSubItemParser, ParseError, ParseResult};
use super::context::BindgenContext;
use super::function::cursor_mangling;
use super::int::IntKind;
use super::item::{Item, ItemId};
use super::ty::TypeKind;

/// A `Var` is our intermediate representation of a variable.
#[derive(Debug)]
pub struct Var {
    /// The name of the variable.
    name: String,
    /// The mangled name of the variable.
    mangled_name: Option<String>,
    /// The type of the variable.
    ty: ItemId,
    /// TODO: support non-integer constants?
    /// The integer value of the variable.
    val: Option<i64>,
    /// Whether this variable is const.
    is_const: bool,
}

impl Var {
    /// Construct a new `Var`.
    pub fn new(name: String,
               mangled: Option<String>,
               ty: ItemId,
               val: Option<i64>,
               is_const: bool)
               -> Var {
        assert!(!name.is_empty());
        Var {
            name: name,
            mangled_name: mangled,
            ty: ty,
            val: val,
            is_const: is_const,
        }
    }

    /// Is this variable `const` qualified?
    pub fn is_const(&self) -> bool {
        self.is_const
    }

    /// The value of this constant variable, if any.
    pub fn val(&self) -> Option<i64> {
        self.val
    }

    /// Get this variable's type.
    pub fn ty(&self) -> ItemId {
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

impl ClangSubItemParser for Var {
    fn parse(cursor: clang::Cursor,
             ctx: &mut BindgenContext)
             -> Result<ParseResult<Self>, ParseError> {
        use clangll::*;
        match cursor.kind() {
            CXCursor_MacroDefinition => {
                let value = parse_int_literal_tokens(&cursor,
                                                     ctx.translation_unit());

                let value = match value {
                    Some(v) => v,
                    None => return Err(ParseError::Continue),
                };

                let name = cursor.spelling();
                if name.is_empty() {
                    warn!("Empty macro name?");
                    return Err(ParseError::Continue);
                }

                if ctx.parsed_macro(&name) {
                    warn!("Duplicated macro definition: {}", name);
                    return Err(ParseError::Continue);
                }
                ctx.note_parsed_macro(name.clone());

                let ty = if value < 0 {
                    Item::builtin_type(TypeKind::Int(IntKind::Int), true, ctx)
                } else if value.abs() > u32::max_value() as i64 {
                    Item::builtin_type(TypeKind::Int(IntKind::ULongLong),
                                       true,
                                       ctx)
                } else {
                    Item::builtin_type(TypeKind::Int(IntKind::UInt), true, ctx)
                };

                Ok(ParseResult::New(Var::new(name,
                                             None,
                                             ty,
                                             Some(value),
                                             true),
                                    Some(cursor)))
            }
            CXCursor_VarDecl => {
                let name = cursor.spelling();
                if name.is_empty() {
                    warn!("Empty constant name?");
                    return Err(ParseError::Continue);
                }

                let ty = cursor.cur_type();

                // XXX this is redundant, remove!
                let is_const = ty.is_const();

                let ty = Item::from_ty(&ty, Some(cursor), None, ctx)
                    .expect("Unable to resolve constant type?");

                // Note: Ty might not be totally resolved yet, see
                // tests/headers/inner_const.hpp
                //
                // That's fine because in that case we know it's not a literal.
                let value = ctx.safe_resolve_type(ty)
                    .and_then(|t| t.safe_canonical_type(ctx))
                    .and_then(|t| if t.is_integer() { Some(t) } else { None })
                    .and_then(|_| {
                        get_integer_literal_from_cursor(&cursor,
                                                        ctx.translation_unit())
                    });

                let mangling = cursor_mangling(&cursor);

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

/// Try and parse the immediately found tokens from an unit (if any) to integers
fn parse_int_literal_tokens(cursor: &clang::Cursor,
                            unit: &clang::TranslationUnit)
                            -> Option<i64> {
    use clangll::{CXToken_Literal, CXToken_Punctuation};

    let tokens = match unit.tokens(cursor) {
        None => return None,
        Some(tokens) => tokens,
    };

    let mut literal = None;
    let mut negate = false;
    for token in tokens.into_iter() {
        match token.kind {
            CXToken_Punctuation if token.spelling == "-" => {
                negate = !negate;
            }
            CXToken_Literal => {
                literal = Some(token.spelling);
                break;
            }
            _ => {
                // Reset values if we found anything else
                negate = false;
                literal = None;
            }
        }
    }

    literal.and_then(|lit| {
            if lit.starts_with("0x") {
                // TODO: try to preserve hex literals?
                i64::from_str_radix(&lit[2..], 16).ok()
            } else if lit == "0" {
                Some(0)
            } else if lit.starts_with("0") {
                i64::from_str_radix(&lit[1..], 8).ok()
            } else {
                lit.parse().ok()
            }
        })
        .map(|lit| if negate { -lit } else { lit })
}

fn get_integer_literal_from_cursor(cursor: &clang::Cursor,
                                   unit: &clang::TranslationUnit)
                                   -> Option<i64> {
    use clangll::*;
    let mut value = None;
    cursor.visit(|c| {
        match c.kind() {
            CXCursor_IntegerLiteral |
            CXCursor_UnaryOperator => {
                value = parse_int_literal_tokens(&c, unit);
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
