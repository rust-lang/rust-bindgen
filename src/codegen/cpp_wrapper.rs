use crate::ir::context::GeneratingStage;
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
    /// Without knowingt the type, we can not calculate the function signature
    /// as a String.
    ArgumentTypeIsNone,

    /// This is valid C++: `struct MyStruct { union { struct {};};};`. Since the
    /// inner struct has no name, we cannot take this structure as a function
    /// argument.
    TypeInsideUnnamedType,

    /// One argument is an anonymous type
    UnnamedType,

    /// Functions with a double underscore prefix are sometimes weird
    DoubleUnderscore,
}

pub fn get_cpp_typename_without_namespace<'a>(
    ctx: &'a BindgenContext,
    item: &'a Item,
) -> Result<&'a str, WhyNoWrapper> {
    let typ = item.kind().expect_type();
    if let Some(name) = typ.name() {
        return Ok(name);
    }
    match typ.kind() {
        TypeKind::ResolvedTypeRef(v) => {
            let name = ctx.resolve_item(v).kind().expect_type().name().unwrap();
            Ok(name)
        }
        TypeKind::Comp(_) => Err(WhyNoWrapper::UnnamedType),
        _ => panic!(),
    }
}

pub fn get_cpp_namespace_prefix(
    ctx: &BindgenContext,
    item: &Item,
) -> Result<String, WhyNoWrapper> {
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
                                //todo: what if the C++ sourcecode is namespace root {...} ?
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
    debug_assert!(ctx.generating_stage() == GeneratingStage::GeneratingCpp);
    let signature = funcitem.expect_function().get_signature(ctx);
    if ctx
        .resolve_item(signature.return_type())
        .kind()
        .expect_type()
        .surely_trivially_relocatable(ctx)
    {
        return Err(WhyNoWrapper::Unnecessary);
    }
    let rettype = get_cpp_typename_with_namespace(
        ctx,
        ctx.resolve_item(signature.return_type()),
    )?;
    let canonical_name = funcitem.canonical_name(ctx);
    if canonical_name.starts_with("__") {
        return Err(WhyNoWrapper::DoubleUnderscore);
    }
    let mut badflag = false;
    let args_string: Vec<String> = signature
        .argument_types()
        .iter()
        .enumerate()
        .map(|(i, &(ref argname, ty))| {
            let argname = argname.as_ref();
            let typ = ctx.resolve_item(ty).expect_type().name();
            if matches!(argname, Some(v) if v == "this") {
                if let None = item {
                    badflag = true;
                    String::new()
                } else {
                    format!("{} *this_ptr, ", item.unwrap().canonical_name(ctx))
                }
            } else if typ == None {
                badflag = true;
                String::new()
            } else {
                format!("{} arg_{},", typ.unwrap(), i)
            }
        })
        .collect();
    if badflag {
        return Err(WhyNoWrapper::ArgumentTypeIsNone);
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
    signature.argument_types().iter().enumerate().for_each(
        |(i, &(ref argname, _))| {
            let argname = argname.as_ref();
            if !matches!(argname, Some(v) if v == "this") {
                args_call.push(format!("arg_{}", i));
            }
        },
    );
    cpp_out.push_str(&format!(
        "void bindgen_wrap_{}({} {} *writeback) {{\n    auto val = {}({});\n    *writeback = val;\n}}\n",
        canonical_name,
        &args_string.join(""),
        rettype,
        callname,
        &args_call.join(", ")
    ));
    Ok(())
}
