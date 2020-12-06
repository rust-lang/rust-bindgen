use crate::ir::context::GeneratingStage;
use crate::ir::function::FunctionSig;
use crate::ir::item::ItemCanonicalName;
use crate::ir::ty::TypeKind;
use crate::BindgenContext;
use crate::Item;

pub fn get_cpp_typename_without_namespace<'a>(
    ctx: &'a BindgenContext,
    item: &'a Item,
) -> &'a str {
    let typ = item.kind().expect_type();
    if let Some(name) = typ.name() {
        return name;
    }
    match typ.kind() {
        TypeKind::ResolvedTypeRef(v) => {
            let name = ctx.resolve_item(v).kind().expect_type().name().unwrap();
            name
        }
        _ => panic!(),
    }
}
pub fn get_cpp_typename_with_namespace(
    ctx: &BindgenContext,
    item: &Item,
) -> String {
    let prefix = {
        let mut prefix = String::new();
        let mut head = item;
        loop {
            head = ctx.resolve_item(head.parent_id());
            let name = head.kind().expect_module().name().unwrap();
            if name == "root" {
                //todo: what if the C++ sourcecode is namespace root {...} ?
                break;
            }
            prefix = format!("{}::{}", name, prefix);
        }
        prefix
    };
    format!(
        "{}{}",
        prefix,
        get_cpp_typename_without_namespace(ctx, item)
    )
}
pub fn cpp_function_wrapper(
    name: &str,
    ctx: &BindgenContext,
    item: Option<&Item>,
    signature: &FunctionSig,
    canonical_name: &String,
    cpp_out: &mut String,
) {
    debug_assert!(ctx.generating_stage() == GeneratingStage::GeneratingCpp);
    if ctx
        .resolve_item(signature.return_type())
        .kind()
        .expect_type()
        .surely_trivially_relocatable(ctx)
    {
        return;
    }
    let rettype = get_cpp_typename_with_namespace(
        ctx,
        ctx.resolve_item(signature.return_type()),
    );
    if !canonical_name.starts_with("__") {
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
                        format!(
                            "{} *this_ptr, ",
                            item.unwrap().canonical_name(ctx)
                        )
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
            return;
        }
        let callname = match item {
            Some(_) => format!("this_ptr->{}", name),
            None => name.to_string(),
        };
        let mut args_call: Vec<String> = Vec::new();
        signature
            .argument_types()
            .iter()
            .enumerate()
            .for_each(|(i, &(ref argname, _))| {
                let argname = argname.as_ref();
                if !matches!(argname, Some(v) if v == "this") {
                    args_call.push(format!("arg_{}", i));
                }
            });
        cpp_out.push_str(&format!(
            "void bindgen_wrap_{}({} {} *writeback) {{\n    auto val = {}({});\n    *writeback = val;\n}}\n",
            &canonical_name,
            &args_string.join(""),
            rettype,
            callname,
            &args_call.join(", ")
        ));
    }
}
