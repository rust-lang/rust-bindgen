use crate::codegen::utils;
use crate::codegen::utils::argument_type_id_to_rust_type;
use crate::ir::comp::MethodKind;
use crate::ir::function::FunctionKind;
use crate::ir::item::ItemCanonicalName;
use crate::ir::item_kind::ItemKind;
use crate::ir::ty::TypeKind;
use crate::BindgenContext;
use crate::Item;

/// This Enum list possible reasons why bindgen did not create a c++ wrapper function:
#[derive(Debug)]
pub enum WhyNoWrapper {
    /// It might be possible to generate the wrapper, but doing so is
    /// unnecessary, because calling the function/method directly should also
    /// work fine.
    Unnecessary,

    /// The type of one function argument is None, i.e. we don't know the type.
    /// Without knowing the type, we can not calculate the function signature
    /// as a String.
    ArgumentTypeIsNone,

    /// This is valid C++: `struct MyStruct { union { struct {};};};`. Since the
    /// inner struct has no name, we cannot take this structure as a function
    /// argument.
    TypeInsideUnnamedType,

    /// One argument or return value is an anonymous type.
    UnnamedType,

    /// One argument or return value is a templated type. We cannot handle
    /// that (yet).
    TemplatedType,

    /// One argument or return value is a function pointer. We cannot handle
    /// that (yet).
    FunctionPointer,

    /// One argument or return value is an array. We cannot handle
    /// that (yet).
    Array,

    /// One argument or return value is an incomplete type.
    IncompleteType,

    /// Functions with a double underscore prefix are sometimes weird.
    DoubleUnderscore,
}

pub fn get_cpp_typename_without_namespace<'a>(
    ctx: &'a BindgenContext,
    item: &'a Item,
) -> Result<String, WhyNoWrapper> {
    let item = item.follow_resolved_type_references(ctx);
    let typ = item.kind().expect_type();
    match typ.kind() {
        TypeKind::TemplateInstantiation(_) => Err(WhyNoWrapper::TemplatedType),
        TypeKind::Comp(compinfo) => {
            if compinfo.is_forward_declaration() {
                Err(WhyNoWrapper::IncompleteType)
            } else {
                match typ.name() {
                    Some(v) => Ok(v.to_owned()),
                    None => Err(WhyNoWrapper::UnnamedType),
                }
            }
        }
        TypeKind::Function(_) => Err(WhyNoWrapper::FunctionPointer),
        TypeKind::Array(_, _) => Err(WhyNoWrapper::Array),

        TypeKind::Pointer(inner) => Ok(format!(
            " *{}",
            get_cpp_typename_without_namespace(ctx, ctx.resolve_item(inner))?
        )),
        TypeKind::Reference(inner) => Ok(format!(
            " &{}",
            get_cpp_typename_without_namespace(ctx, ctx.resolve_item(inner))?
        )),
        TypeKind::Void => Ok("void".to_owned()),
        _ => Ok(typ.name().unwrap().to_owned()),
    }
}

pub fn get_cpp_namespace_prefix(
    ctx: &BindgenContext,
    item: &Item,
) -> Result<String, WhyNoWrapper> {
    let item = item.follow_resolved_type_references(ctx);
    let prefix = {
        let mut prefix = String::new();
        let mut head = item;
        loop {
            head = ctx.resolve_item(head.parent_id());
            prefix = match head.kind() {
                ItemKind::Module(v) => {
                    match v.name() {
                        None => prefix, // v.name() is None if it is an unnamed namespace
                        Some(name) => {
                            if name == "root" {
                                //TODO: what if the C++ sourcecode is: namespace root {...} ?
                                break;
                            }
                            format!("{}::{}", name, prefix)
                        }
                    }
                },
                ItemKind::Type(v) => {
                    match v.name() {
                        None => {return Err(WhyNoWrapper::TypeInsideUnnamedType);},
                        Some(v) => format!("{}::{}", v, prefix),
                    }
                },
                ItemKind::Function(_) => panic!("Bindgen does not support types declared inside of functions."),
                ItemKind::Var(_) => panic!("Bindgen does not support types declared inside of variables."),
            };
        }
        prefix
    };
    Ok(prefix)
}

pub fn get_cpp_typename_with_namespace(
    ctx: &BindgenContext,
    item: &Item,
) -> Result<String, WhyNoWrapper> {
    let prefix = get_cpp_namespace_prefix(ctx, item)?;
    Ok(format!(
        "{}{}",
        prefix,
        get_cpp_typename_without_namespace(ctx, item)?
    ))
}

pub fn cpp_function_wrapper(
    funcitem: &Item,
    ctx: &BindgenContext,
    item: Option<&Item>,
    cpp_out: &mut String,
) -> Result<(), WhyNoWrapper> {
    let signature = funcitem.expect_function().get_signature(ctx);

    let using_wrapped_return = !ctx
        .resolve_item(signature.return_type())
        .kind()
        .expect_type()
        .surely_trivially_relocatable(ctx);
    let rettype = get_cpp_typename_with_namespace(
        ctx,
        ctx.resolve_item(signature.return_type()),
    )?;
    let canonical_name = funcitem.canonical_name(ctx);
    if funcitem.expect_function().name().starts_with("__") {
        return Err(WhyNoWrapper::DoubleUnderscore);
    }
    let mut badflag = None;
    let (args_string, boxed): (Vec<String>, Vec<bool>) = signature
        .argument_types()
        .iter()
        .enumerate()
        .map(|(i, &(ref argname, ty))| {
            let arg_ty = argument_type_id_to_rust_type(ctx, ty, true);

            let argname = argname.as_ref();
            let typ = match get_cpp_typename_with_namespace(
                ctx,
                ctx.resolve_item(ty),
            ) {
                Ok(v) => v,
                Err(e) => {
                    badflag = Some(e);
                    return (String::new(), false);
                }
            };
            if matches!(argname, Some(v) if v == "this") {
                if let None = item {
                    badflag = Some(WhyNoWrapper::ArgumentTypeIsNone);
                    (String::new(), false)
                } else {
                    (
                        format!(
                            "{} *this_ptr",
                            get_cpp_typename_with_namespace(ctx, item.unwrap()).unwrap()
                        ),
                        false,
                    )
                }
            } else {
                match arg_ty {
                    utils::TypeName::Void => panic!(),
                    utils::TypeName::NotBoxable(_) => {
                        (format!("{} arg_{}", typ, i), false)
                    }
                    utils::TypeName::Boxable {
                        unboxed: _,
                        boxed: _,
                    } => (format!("{} *arg_{}", typ, i), true),
                }
            }
        })
        .unzip();
    if !using_wrapped_return && !boxed.iter().any(|el| *el) {
        return Err(WhyNoWrapper::Unnecessary);
    }
    if let Some(e) = badflag {
        return Err(e);
    }
    let callname = match item {
        Some(_) => {
            format!("this_ptr->{}", funcitem.kind().expect_function().name())
        }
        None => format!(
            "{}{}",
            get_cpp_namespace_prefix(ctx, funcitem)?,
            funcitem.kind().expect_function().name()
        ),
    };
    let mut args_call: Vec<String> = Vec::new();
    signature
        .argument_types()
        .iter()
        .zip(boxed.iter())
        .enumerate()
        .for_each(|(i, (&(ref argname, _), boxed))| {
            let argname = argname.as_ref();
            if !matches!(argname, Some(v) if v == "this") {
                if *boxed {
                    args_call.push(format!("*arg_{}", i));
                } else {
                    args_call.push(format!("arg_{}", i));
                }
            }
        });
    let is_constructor = funcitem.expect_function().kind() ==
        FunctionKind::Method(MethodKind::Constructor);
    if is_constructor {
        cpp_out.push_str(&format!(
            "void bindgen_wrap_{}({}) {{\n    *this_ptr = {}({});\n}}\n",
            canonical_name,
            &args_string.join(", "),
            item.unwrap().canonical_name(ctx),
            &args_call.join(", ")
        ));
        // TODO(volker): use emplacement new:
        // cpp_out.push_str(&format!(
        //     "void bindgen_wrap_{}({}) {{\n    {} *val = new(this_ptr) {}({});\n}}\n",
        //     canonical_name,
        //     &args_string.join(", "),
        //     item.unwrap().canonical_name(ctx),
        //     item.unwrap().canonical_name(ctx),
        //     &args_call.join(", ")
        // ));
    } else if using_wrapped_return {
        cpp_out.push_str(&format!(
            "void bindgen_wrap_{}({}{} {} *writeback) {{\n    auto val = {}({});\n    *writeback = val;\n}}\n",
            canonical_name,
            &args_string.join(", "),
            if args_string.len() == 0 {
                ""
            } else {
                ", "
            },
            rettype,
            callname,
            &args_call.join(", ")
        ));
    } else {
        cpp_out.push_str(&format!(
            "{} bindgen_wrap_{}({}) {{\n    return {}({});\n}}\n",
            rettype,
            canonical_name,
            &args_string.join(", "),
            callname,
            &args_call.join(", ")
        ));
    }
    Ok(())
}
