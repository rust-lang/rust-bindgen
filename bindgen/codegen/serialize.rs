use std::io::{self, Write};

use crate::callbacks::IntKind;

use crate::ir::context::{BindgenContext, TypeId};
use crate::ir::function::{Function, FunctionKind};
use crate::ir::item::Item;
use crate::ir::item_kind::ItemKind;
use crate::ir::ty::{FloatKind, Type, TypeKind};

pub(crate) trait CSerialize {
    fn serialize<W: Write>(
        &self,
        ctx: &BindgenContext,
        writer: &mut W,
    ) -> io::Result<()>;
}

impl CSerialize for Item {
    fn serialize<W: Write>(
        &self,
        ctx: &BindgenContext,
        writer: &mut W,
    ) -> io::Result<()> {
        match self.kind() {
            ItemKind::Function(func) => func.serialize(ctx, writer),
            kind => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Cannot serialize item kind {:?}", kind),
                ));
            }
        }
    }
}

impl CSerialize for Function {
    fn serialize<W: Write>(
        &self,
        ctx: &BindgenContext,
        writer: &mut W,
    ) -> io::Result<()> {
        if self.kind() != FunctionKind::Function {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Cannot serialize function kind {:?}", self.kind()),
            ));
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
        ret_ty.serialize(ctx, writer)?;
        write!(writer, " {}(", wrap_name)?;
        serialize_sep(
            ", ",
            args.iter(),
            ctx,
            writer,
            |(name, type_id), ctx, buf| {
                type_id.serialize(ctx, buf)?;
                write!(buf, " {}", name)
            },
        )?;
        writeln!(writer, ") asm(\"{}\");", wrap_name)?;

        // Write `ret_ty wrap_name(args) { return name(arg_names)' }`
        ret_ty.serialize(ctx, writer)?;
        write!(writer, " {}(", wrap_name)?;
        serialize_sep(
            ", ",
            args.iter(),
            ctx,
            writer,
            |(name, type_id), _, buf| {
                type_id.serialize(ctx, buf)?;
                write!(buf, "{}", name)
            },
        )?;
        write!(writer, ") {{ return {}(", name)?;
        serialize_sep(", ", args.iter(), ctx, writer, |(name, _), _, buf| {
            write!(buf, "{}", name)
        })?;
        writeln!(writer, "); }}")?;

        Ok(())
    }
}

impl CSerialize for TypeId {
    fn serialize<W: Write>(
        &self,
        ctx: &BindgenContext,
        writer: &mut W,
    ) -> io::Result<()> {
        ctx.resolve_type(*self).serialize(ctx, writer)
    }
}

impl CSerialize for Type {
    fn serialize<W: Write>(
        &self,
        ctx: &BindgenContext,
        writer: &mut W,
    ) -> io::Result<()> {
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
                int_kind => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Cannot serialize integer kind {:?}", int_kind),
                    ))
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
            TypeKind::Alias(type_id) => { type_id.serialize(ctx, writer) }?,
            TypeKind::TemplateAlias(type_id, params) => {
                type_id.serialize(ctx, writer)?;
                write!(writer, "<")?;
                serialize_sep(
                    ", ",
                    params.iter(),
                    ctx,
                    writer,
                    TypeId::serialize,
                )?;
                write!(writer, ">")?
            }
            TypeKind::Array(type_id, length) => {
                type_id.serialize(ctx, writer)?;
                write!(writer, " [{}]", length)?
            }
            TypeKind::Function(signature) => {
                signature.return_type().serialize(ctx, writer)?;
                write!(writer, " (")?;
                serialize_sep(
                    ", ",
                    signature.argument_types().iter(),
                    ctx,
                    writer,
                    |(name, type_id), ctx, buf| {
                        type_id.serialize(ctx, buf)?;

                        if let Some(name) = name {
                            write!(buf, "{}", name)?;
                        }

                        Ok(())
                    },
                )?;
                write!(writer, ")")?
            }
            TypeKind::ResolvedTypeRef(type_id) => {
                type_id.serialize(ctx, writer)?
            }
            TypeKind::Pointer(type_id) => {
                type_id.serialize(ctx, writer)?;
                write!(writer, "*")?;
            }
            ty => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Cannot serialize type kind {:?}", ty),
                ))
            }
        };

        Ok(())
    }
}

fn serialize_sep<
    W: io::Write,
    F: Fn(I::Item, &BindgenContext, &mut W) -> io::Result<()>,
    I: Iterator,
>(
    sep: &'static str,
    mut iter: I,
    ctx: &BindgenContext,
    buf: &mut W,
    f: F,
) -> io::Result<()> {
    if let Some(item) = iter.next() {
        f(item, ctx, buf)?;
        for item in iter {
            write!(buf, "{}", sep)?;
            f(item, ctx, buf)?;
        }
    }

    Ok(())
}
