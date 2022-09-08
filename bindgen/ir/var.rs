//! Intermediate representation of variables.

use std::borrow::Cow;
use std::io;

use super::context::{BindgenContext, TypeId};
use super::dot::DotAttributes;
use super::function::cursor_mangling;
use super::int::IntKind;
use super::item::Item;
use super::ty::TypeKind;
use crate::callbacks::{ItemInfo, ItemKind};
use crate::clang;
use crate::parse::{ClangSubItemParser, ParseError, ParseResult};

/// The type for a constant variable.
#[derive(Debug)]
pub(crate) enum VarType {
    /// A boolean.
    Bool(bool),
    /// An integer.
    Int(i128),
    /// A floating point number.
    Float(f64),
    /// A character.
    #[allow(unused)]
    Char(u8),
    /// A string, not necessarily well-formed utf-8.
    String(Vec<u8>),
}

/// A `Var` is our intermediate representation of a variable.
#[derive(Debug)]
pub(crate) struct Var {
    /// The name of the variable.
    name: String,
    /// The mangled name of the variable.
    mangled_name: Option<String>,
    /// The link name of the variable.
    link_name: Option<String>,
    /// The type of the variable.
    ty: TypeId,
    /// The value of the variable, that needs to be suitable for `ty`.
    val: Option<VarType>,
    /// Whether this variable is const.
    is_const: bool,
}

impl Var {
    /// Construct a new `Var`.
    pub(crate) fn new(
        name: String,
        mangled_name: Option<String>,
        link_name: Option<String>,
        ty: TypeId,
        val: Option<VarType>,
        is_const: bool,
    ) -> Self {
        assert!(!name.is_empty());
        Self {
            name,
            mangled_name,
            link_name,
            ty,
            val,
            is_const,
        }
    }

    /// Is this variable `const` qualified?
    pub(crate) fn is_const(&self) -> bool {
        self.is_const
    }

    /// The value of this constant variable, if any.
    pub(crate) fn val(&self) -> Option<&VarType> {
        self.val.as_ref()
    }

    /// Get this variable's type.
    pub(crate) fn ty(&self) -> TypeId {
        self.ty
    }

    /// Get this variable's name.
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    /// Get this variable's mangled name.
    pub(crate) fn mangled_name(&self) -> Option<&str> {
        self.mangled_name.as_deref()
    }

    /// Get this variable's link name.
    pub fn link_name(&self) -> Option<&str> {
        self.link_name.as_deref()
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

impl ClangSubItemParser for Var {
    fn parse(
        cursor: clang::Cursor,
        ctx: &mut BindgenContext,
    ) -> Result<ParseResult<Self>, ParseError> {
        use clang_sys::{
            CXCursor_VarDecl, CXLinkage_External, CXType_Auto,
            CXType_ConstantArray, CXType_IncompleteArray, CXType_Unexposed,
        };

        if cursor.kind() != CXCursor_VarDecl {
            return Err(ParseError::Continue);
        }

        let mut name = cursor.spelling();

        if cursor.linkage() == CXLinkage_External {
            if let Some(nm) = ctx.options().last_callback(|callbacks| {
                callbacks.generated_name_override(ItemInfo {
                    name: name.as_str(),
                    kind: ItemKind::Var,
                })
            }) {
                name = nm;
            }
        }
        // No more changes to name
        let name = name;

        if name.is_empty() {
            warn!("Empty constant name?");
            return Err(ParseError::Continue);
        }

        // If a variable exists, there cannot be a variable-like macro with the same name,
        // except if the macro expands to its own name, in which case it is useless anyways.
        ctx.macro_set.undefine_var_macro(&name);

        let link_name = ctx.options().last_callback(|callbacks| {
            callbacks.generated_link_name_override(ItemInfo {
                name: name.as_str(),
                kind: ItemKind::Var,
            })
        });

        let ty = cursor.cur_type();

        // TODO(emilio): do we have to special-case constant arrays in
        // some other places?
        let is_const = ty.is_const() ||
            ([CXType_ConstantArray, CXType_IncompleteArray]
                .contains(&ty.kind()) &&
                ty.elem_type()
                    .map_or(false, |element| element.is_const()));

        let ty = match Item::from_ty(&ty, cursor, None, ctx) {
            Ok(ty) => ty,
            Err(e) => {
                assert!(
                    matches!(ty.kind(), CXType_Auto | CXType_Unexposed),
                    "Couldn't resolve constant type, and it \
                             wasn't an nondeductible auto type or unexposed \
                             type!"
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
                val = get_integer_literal_from_cursor(&cursor);
            }

            val.map(|val| {
                match kind {
                    IntKind::Bool => VarType::Bool(val != 0),
                    // IntKind::Char { .. } => VarType::Char(val as u8),
                    _ => VarType::Int(val),
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
        let var = Self::new(name, mangling, link_name, ty, value, is_const);

        Ok(ParseResult::New(var, Some(cursor)))
    }
}

fn parse_int_literal_tokens(cursor: &clang::Cursor) -> Option<i128> {
    let tokens = cursor.tokens().iter().collect::<Vec<_>>();
    let tokens = tokens
        .iter()
        .map(|token| {
            let spelling = token.spelling();
            match spelling.to_str() {
                Ok(token) => Cow::Borrowed(token),
                Err(_) => {
                    let token = spelling.to_string_lossy();
                    warn!(
                        "Lossy conversion of non-UTF8 token {:?} to {:?}.",
                        spelling, token
                    );
                    token
                }
            }
        })
        .collect::<Vec<_>>();

    let macro_set = cmacro::MacroSet::new();
    let cmacro_tokens = macro_set.expand(&tokens).ok()?;

    // TODO(emilio): We can try to parse other kinds of literals.
    match cmacro::LitInt::parse(&cmacro_tokens) {
        Ok((_, cmacro::LitInt { value, .. })) => Some(value),
        _ => None,
    }
}

fn get_integer_literal_from_cursor(cursor: &clang::Cursor) -> Option<i128> {
    use clang_sys::*;
    let mut value = None;
    cursor.visit(|c| {
        match c.kind() {
            CXCursor_IntegerLiteral | CXCursor_UnaryOperator => {
                value = parse_int_literal_tokens(&c);
            }
            CXCursor_UnexposedExpr => {
                value = get_integer_literal_from_cursor(&c);
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
