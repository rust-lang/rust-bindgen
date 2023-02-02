use std::io::Write;

use crate::callbacks::IntKind;

use crate::ir::comp::CompKind;
use crate::ir::context::{BindgenContext, TypeId};
use crate::ir::function::{Function, FunctionKind};
use crate::ir::item::Item;
use crate::ir::item::ItemCanonicalName;
use crate::ir::item_kind::ItemKind;
use crate::ir::ty::{FloatKind, Type, TypeKind};

use super::CodegenError;

fn get_loc(item: &Item) -> String {
    item.location()
        .map(|x| x.to_string())
        .unwrap_or_else(|| "unknown".to_owned())
}

pub(crate) trait CSerialize<'a, 'ctx> {
    type Extra;

    fn serialize<W: Write>(
        &self,
        ctx: &'ctx BindgenContext,
        arg: Self::Extra,
        writer: &mut W,
    ) -> Result<(), CodegenError>;
}

impl<'a, 'ctx> CSerialize<'a, 'ctx> for Item {
    type Extra = ();

    fn serialize<W: Write>(
        &self,
        ctx: &'ctx BindgenContext,
        (): Self::Extra,
        writer: &mut W,
    ) -> Result<(), CodegenError> {
        match self.kind() {
            ItemKind::Function(func) => func.serialize(ctx, self, writer),
            kind => {
                return Err(CodegenError::Serialize {
                    msg: format!("Cannot serialize item kind {:?}", kind),
                    loc: get_loc(self),
                });
            }
        }
    }
}

impl<'a, 'ctx> CSerialize<'a, 'ctx> for Function {
    type Extra = &'a Item;

    fn serialize<W: Write>(
        &self,
        ctx: &'ctx BindgenContext,
        item: Self::Extra,
        writer: &mut W,
    ) -> Result<(), CodegenError> {
        if self.kind() != FunctionKind::Function {
            return Err(CodegenError::Serialize {
                msg: format!(
                    "Cannot serialize function kind {:?}",
                    self.kind(),
                ),
                loc: get_loc(item),
            });
        }

        let signature = match ctx.resolve_type(self.signature()).kind() {
            TypeKind::Function(signature) => signature,
            _ => unreachable!(),
        };

        let name = self.name();

        // Function argoments stored as `(name, type_id)` tuples.
        let args = {
            let mut count = 0;

            signature
                .argument_types()
                .iter()
                .cloned()
                .map(|(opt_name, type_id)| {
                    (
                        opt_name.unwrap_or_else(|| {
                            let name = format!("arg_{}", count);
                            count += 1;
                            name
                        }),
                        type_id,
                    )
                })
                .collect::<Vec<_>>()
        };

        // The name used for the wrapper self.
        let wrap_name = format!("{}{}", name, ctx.wrap_static_fns_suffix());
        // The function's return type
        let ret_ty = signature.return_type();

        // Write `ret_ty wrap_name(args) asm("wrap_name");`
        ret_ty.serialize(ctx, &mut None, writer)?;
        write!(writer, " {}(", wrap_name)?;
        serialize_sep(
            ", ",
            args.iter(),
            ctx,
            writer,
            |(name, type_id), ctx, buf| {
                type_id.serialize(ctx, &mut Some(name.as_str()), buf)
            },
        )?;
        writeln!(writer, ") asm(\"{}\");", wrap_name)?;

        // Write `ret_ty wrap_name(args) { return name(arg_names)' }`
        ret_ty.serialize(ctx, &mut None, writer)?;
        write!(writer, " {}(", wrap_name)?;
        serialize_sep(
            ", ",
            args.iter(),
            ctx,
            writer,
            |(name, type_id), _, buf| {
                type_id.serialize(ctx, &mut Some(name.as_str()), buf)
            },
        )?;
        write!(writer, ") {{ return {}(", name)?;
        serialize_sep(", ", args.iter(), ctx, writer, |(name, _), _, buf| {
            write!(buf, "{}", name).map_err(From::from)
        })?;
        writeln!(writer, "); }}")?;

        Ok(())
    }
}

impl<'a> CSerialize<'a, 'a> for TypeId {
    type Extra = &'a mut Option<&'a str>;

    fn serialize<W: Write>(
        &self,
        ctx: &'a BindgenContext,
        arg: Self::Extra,
        writer: &mut W,
    ) -> Result<(), CodegenError> {
        let item = ctx.resolve_item(*self);
        item.expect_type().serialize(ctx, (item, arg), writer)
    }
}

impl<'a> CSerialize<'a, 'a> for Type {
    type Extra = (&'a Item, &'a mut Option<&'a str>);

    fn serialize<W: Write>(
        &self,
        ctx: &'a BindgenContext,
        (item, arg): Self::Extra,
        writer: &mut W,
    ) -> Result<(), CodegenError> {
        match self.kind() {
            TypeKind::Void => write!(writer, "void")?,
            TypeKind::NullPtr => write!(writer, "nullptr_t")?,
            TypeKind::Int(int_kind) => match int_kind {
                IntKind::Bool => write!(writer, "bool")?,
                IntKind::SChar => write!(writer, "signed char")?,
                IntKind::UChar => write!(writer, "unsigned char")?,
                IntKind::WChar => write!(writer, "wchar_t")?,
                IntKind::Short => write!(writer, "short")?,
                IntKind::UShort => write!(writer, "unsigned short")?,
                IntKind::Int => write!(writer, "int")?,
                IntKind::UInt => write!(writer, "unsigned int")?,
                IntKind::Long => write!(writer, "long")?,
                IntKind::ULong => write!(writer, "unsigned long")?,
                IntKind::LongLong => write!(writer, "long long")?,
                IntKind::ULongLong => write!(writer, "unsigned long long")?,
                IntKind::Char { .. } => write!(writer, "char")?,
                int_kind => {
                    return Err(CodegenError::Serialize {
                        msg: format!(
                            "Cannot serialize integer kind {:?}",
                            int_kind
                        ),
                        loc: get_loc(item),
                    })
                }
            },
            TypeKind::Float(float_kind) => match float_kind {
                FloatKind::Float => write!(writer, "float")?,
                FloatKind::Double => write!(writer, "double")?,
                FloatKind::LongDouble => write!(writer, "long double")?,
                FloatKind::Float128 => write!(writer, "__float128")?,
            },
            TypeKind::Complex(float_kind) => match float_kind {
                FloatKind::Float => write!(writer, "float complex")?,
                FloatKind::Double => write!(writer, "double complex")?,
                FloatKind::LongDouble => write!(writer, "long double complex")?,
                FloatKind::Float128 => write!(writer, "__complex128")?,
            },
            TypeKind::Alias(type_id) => {
                type_id.serialize(ctx, &mut None, writer)?
            }
            TypeKind::TemplateAlias(type_id, params) => {
                type_id.serialize(ctx, &mut None, writer)?;
                write!(writer, "<")?;
                serialize_sep(
                    ", ",
                    params.iter(),
                    ctx,
                    writer,
                    |type_id, ctx, writer| {
                        type_id.serialize(ctx, &mut None, writer)
                    },
                )?;
                write!(writer, ">")?
            }
            TypeKind::Array(type_id, length) => {
                type_id.serialize(ctx, &mut None, writer)?;
                write!(writer, " [{}]", length)?
            }
            TypeKind::Function(signature) => {
                signature.return_type().serialize(ctx, &mut None, writer)?;

                if let Some(name) = arg.take() {
                    write!(writer, " ({})", name)?;
                }

                write!(writer, " (")?;
                serialize_sep(
                    ", ",
                    signature.argument_types().iter(),
                    ctx,
                    writer,
                    |(arg, type_id), ctx, buf| {
                        type_id.serialize(ctx, &mut arg.as_ref().map(|a| a.as_str()), buf)
                    },
                )?;
                write!(writer, ")")?
            }
            TypeKind::ResolvedTypeRef(type_id) => {
                return type_id.serialize(ctx, arg, writer);
            }
            TypeKind::Pointer(type_id) => {
                let mut name = "*".to_owned();
                if let Some(arg) = arg {
                    name += arg;
                }
                type_id.serialize(ctx, &mut Some(name.as_str()), writer)?;

                return Ok(());
            }
            TypeKind::Comp(comp_info) => {
                let name = item.canonical_name(ctx);

                match comp_info.kind() {
                    CompKind::Struct => write!(writer, "struct {}", name)?,
                    CompKind::Union => write!(writer, "union {}", name)?,
                };
            }
            ty => {
                return Err(CodegenError::Serialize {
                    msg: format!("Cannot serialize type kind {:?}", ty),
                    loc: get_loc(item),
                })
            }
        };

        if let Some(arg) = arg {
            write!(writer, " {}", arg)?;
        }

        Ok(())
    }
}

fn serialize_sep<
    W: Write,
    F: Fn(I::Item, &BindgenContext, &mut W) -> Result<(), CodegenError>,
    I: Iterator,
>(
    sep: &'static str,
    mut iter: I,
    ctx: &BindgenContext,
    buf: &mut W,
    f: F,
) -> Result<(), CodegenError> {
    if let Some(item) = iter.next() {
        f(item, ctx, buf)?;
        for item in iter {
            write!(buf, "{}", sep)?;
            f(item, ctx, buf)?;
        }
    }

    Ok(())
}
