use super::item::{Item, ItemId};
use super::ty::TypeKind;
use super::context::BindgenContext;
use parse::{ClangItemParser, ParseError};
use clang;

#[derive(Debug)]
pub struct Enum {
    /// The representation used for this enum.
    /// Should be an IntKind type.
    ///
    /// It's None if the enum is a forward declaration and isn't defined
    /// anywhere else, see tests/headers/func_ptr_in_struct.h
    repr: Option<ItemId>,
    /// The different variants, with explicit values.
    variants: Vec<EnumVariant>,
}

impl Enum {
    pub fn new(repr: Option<ItemId>, variants: Vec<EnumVariant>) -> Self {
        Enum {
            repr: repr,
            variants: variants,
        }
    }

    pub fn repr(&self) -> Option<ItemId> {
        self.repr
    }

    pub fn variants(&self) -> &[EnumVariant] {
        &self.variants
    }

    pub fn from_ty(ty: &clang::Type,
                   ctx: &mut BindgenContext) -> Result<Self, ParseError> {
        use clangll::*;
        if ty.kind() != CXType_Enum {
            return Err(ParseError::Continue);
        }

        let declaration = ty.declaration().canonical();
        let repr = Item::from_ty(&declaration.enum_type(), None, None, ctx).ok();
        let mut variants = vec![];

        let is_signed = match repr {
            Some(repr) => {
                let repr_type = ctx.resolve_type(repr);
                match *repr_type.canonical_type(ctx).kind() {
                    TypeKind::Int(ref int_kind) => int_kind.is_signed(),
                    ref other => panic!("Since when enums can be non-integers? {:?}", other),
                }
            }
            // Assume signedness since the default type by the C standard is an
            // int.
            None => true,
        };

        declaration.visit(|cursor, _| {
            if cursor.kind() == CXCursor_EnumConstantDecl {
                let name = cursor.spelling();
                let comment = cursor.raw_comment();
                let val = if is_signed {
                    EnumVariantValue::Signed(cursor.enum_val_signed())
                } else {
                    EnumVariantValue::Unsigned(cursor.enum_val_unsigned())
                };
                variants.push(EnumVariant::new(name, comment, val));
            }
            CXChildVisit_Continue
        });

        Ok(Enum::new(repr, variants))
    }
}

/// A single enum variant, to be contained only in an enum.
#[derive(Debug)]
pub struct EnumVariant {
    /// The name of the variant.
    name: String,
    /// An optional doc comment.
    comment: Option<String>,
    /// The integer value of the variant.
    val: EnumVariantValue,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnumVariantValue {
    Signed(i64),
    Unsigned(u64),
}

impl EnumVariant {
    pub fn new(name: String, comment: Option<String>, val: EnumVariantValue) -> Self {
        EnumVariant {
            name: name,
            comment: comment,
            val: val,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn val(&self) -> EnumVariantValue {
        self.val
    }
}
