use crate::ir::context::GeneratingStage;
use crate::ir::function::FunctionSig;
use crate::ir::item::ItemCanonicalName;
use crate::ir::ty::{TypeKind};
use crate::BindgenContext;
use crate::Item;

pub fn get_cpp_typename(ctx: &BindgenContext, item: &Item, namespaces: bool) -> String {
    let prefix = if namespaces {
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
    } else {
        String::new()
    };

    // TODO: Codereview: Are there types, where the unwrap() or panic!() below crash bindgen?
    let typ = item.kind().expect_type();
    if let Some(name) = typ.name() {
        return format!("{}{}", prefix, name);
    }
    match typ.kind() {
        TypeKind::ResolvedTypeRef(v) => {
            let name = ctx.resolve_item(v).kind().expect_type().name().unwrap();
            format!("{}{}", prefix, name)
        }
        _ => panic!(),
    }
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
        .surely_trivially_relocatable()
    {
        return;
    }
    let rettype = get_cpp_typename(ctx, ctx.resolve_item(signature.return_type()), true);
    if !canonical_name.starts_with("__") {
        let mut badflag = false;
        let args_string: Vec<String> = signature
            .argument_types()
            .iter()
            .map(|&(ref argname, ty)| {
                let argname = argname.as_ref().unwrap();
                let typ = ctx.resolve_item(ty).expect_type().name();
                if argname == "this" {
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
                    format!("{} {},", typ.unwrap(), argname)
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
            .for_each(|&(ref argname, _)| {
                let argname = argname.as_ref().unwrap();
                if argname != "this" {
                    args_call.push(format!("{}", argname));
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
