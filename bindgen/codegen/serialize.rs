use std::fmt::{self, Write};

use crate::callbacks::IntKind;
use crate::DEFAULT_NON_EXTERN_FNS_SUFFIX;

use crate::ir::context::{BindgenContext, TypeId};
use crate::ir::function::{Function, FunctionKind};
use crate::ir::ty::{FloatKind, TypeKind};

#[derive(Debug)]
pub(crate) enum Error {
    Serialize(String),
    Fmt(fmt::Error),
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Self::Fmt(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::Serialize(err)
    }
}

#[derive(Debug)]
pub(crate) struct CItem {
    code: String,
}

impl CItem {
    pub(crate) fn from_function(
        function: &Function,
        ctx: &BindgenContext,
    ) -> Result<Self, Error> {
        if function.kind() != FunctionKind::Function {
            return Err(Error::Serialize(format!(
                "Cannot serialize function kind {:?}",
                function.kind()
            )));
        }
        let signature = match ctx.resolve_type(function.signature()).kind() {
            TypeKind::Function(signature) => signature,
            _ => unreachable!(),
        };

        let mut code = String::new();

        let mut count = 0;

        let name = function.name();
        let args = signature
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
            .collect::<Vec<_>>();

        serialize_type(signature.return_type(), ctx, &mut code)?;
        write!(
            code,
            " {}{}(",
            name,
            ctx.options()
                .wrap_static_fns_suffix
                .as_deref()
                .unwrap_or(DEFAULT_NON_EXTERN_FNS_SUFFIX)
        )?;
        serialize_sep(
            ", ",
            args.iter(),
            ctx,
            &mut code,
            |(name, type_id), ctx, buf| {
                serialize_type(*type_id, ctx, buf)?;
                write!(buf, " {}", name).map_err(Error::from)
            },
        )?;
        write!(code, ") {{ return {}(", name)?;
        serialize_sep(
            ", ",
            args.iter(),
            ctx,
            &mut code,
            |(name, _), _, buf| write!(buf, "{}", name).map_err(Error::from),
        )?;
        write!(code, "); }}")?;

        Ok(Self { code })
    }

    pub(crate) fn code(&self) -> &str {
        self.code.as_ref()
    }
}

fn serialize_sep<
    W: fmt::Write,
    F: Fn(I::Item, &BindgenContext, &mut W) -> Result<(), Error>,
    I: Iterator,
>(
    sep: &'static str,
    mut iter: I,
    ctx: &BindgenContext,
    buf: &mut W,
    f: F,
) -> Result<(), Error> {
    if let Some(item) = iter.next() {
        f(item, ctx, buf)?;

        for item in iter {
            write!(buf, "{}", sep)?;
            f(item, ctx, buf)?;
        }
    }

    Ok(())
}

fn serialize_type<W: fmt::Write>(
    type_id: TypeId,
    ctx: &BindgenContext,
    buf: &mut W,
) -> Result<(), Error> {
    match ctx.resolve_type(type_id).kind() {
        TypeKind::Void => write!(buf, "void")?,
        TypeKind::NullPtr => write!(buf, "nullptr_t")?,
        TypeKind::Int(int_kind) => match int_kind {
            IntKind::Bool => write!(buf, "bool")?,
            IntKind::SChar => write!(buf, "signed char")?,
            IntKind::UChar => write!(buf, "unsigned char")?,
            IntKind::WChar => write!(buf, "wchar_t")?,
            IntKind::Short => write!(buf, "short")?,
            IntKind::UShort => write!(buf, "unsigned short")?,
            IntKind::Int => write!(buf, "int")?,
            IntKind::UInt => write!(buf, "unsigned int")?,
            IntKind::Long => write!(buf, "long")?,
            IntKind::ULong => write!(buf, "unsigned long")?,
            IntKind::LongLong => write!(buf, "long long")?,
            IntKind::ULongLong => write!(buf, "unsigned long long")?,
            int_kind => {
                return Err(Error::Serialize(format!(
                    "Cannot serialize integer kind {:?}",
                    int_kind
                )))
            }
        },
        TypeKind::Float(float_kind) => match float_kind {
            FloatKind::Float => write!(buf, "float")?,
            FloatKind::Double => write!(buf, "double")?,
            FloatKind::LongDouble => write!(buf, "long double")?,
            FloatKind::Float128 => write!(buf, "__float128")?,
        },
        TypeKind::Complex(float_kind) => match float_kind {
            FloatKind::Float => write!(buf, "float complex")?,
            FloatKind::Double => write!(buf, "double complex")?,
            FloatKind::LongDouble => write!(buf, "long double complex")?,
            FloatKind::Float128 => write!(buf, "__complex128")?,
        },
        TypeKind::Alias(type_id) => serialize_type(*type_id, ctx, buf)?,
        TypeKind::TemplateAlias(type_id, params) => {
            serialize_type(*type_id, ctx, buf)?;
            write!(buf, "<")?;
            serialize_sep(
                ", ",
                params.iter().copied(),
                ctx,
                buf,
                serialize_type,
            )?;
            write!(buf, ">")?
        }
        TypeKind::Array(type_id, length) => {
            serialize_type(*type_id, ctx, buf)?;
            write!(buf, " [{}]", length)?
        }
        TypeKind::Function(signature) => {
            serialize_type(signature.return_type(), ctx, buf)?;
            write!(buf, " (")?;
            serialize_sep(
                ", ",
                signature.argument_types().iter(),
                ctx,
                buf,
                |(name, type_id), ctx, buf| {
                    serialize_type(*type_id, ctx, buf)?;

                    if let Some(name) = name {
                        write!(buf, "{}", name)?;
                    }

                    Ok(())
                },
            )?;
            write!(buf, ")")?
        }
        TypeKind::ResolvedTypeRef(type_id) => {
            serialize_type(*type_id, ctx, buf)?
        }
        ty => {
            return Err(Error::Serialize(format!(
                "Cannot serialize type kind {:?}",
                ty
            )))
        }
    };

    Ok(())
}