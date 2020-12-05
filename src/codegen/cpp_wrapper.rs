use crate::codegen::utils;
use crate::codegen::ToRustTyOrOpaque;
use crate::codegen::TryToRustTy;
use crate::ir::context::{GeneratingStage, TypeId};
use crate::ir::function::FunctionSig;
use crate::ir::item::ItemCanonicalName;
use crate::ir::ty::TypeKind;
use crate::BindgenContext;
use crate::Item;

fn get_cpp_typename(ctx: &BindgenContext, id: TypeId) -> &str {
    // TODO: Codereview: Are there types, where the unwrap() or panic!() below crash bindgen?
    let temp = ctx.resolve_item(id).kind().expect_type();
    if let Some(name) = temp.name() {
        return name;
    }
    match temp.kind() {
        TypeKind::ResolvedTypeRef(v) => {
            ctx.resolve_item(v).kind().expect_type().name().unwrap()
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
    let rettype = get_cpp_typename(ctx, signature.return_type());
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
            "void bindgen_wrap_{}({} {} *writeback) {{
auto val = {}({});
*writeback = val;
}}
",
            &canonical_name,
            &args_string.join(""),
            rettype,
            callname,
            &args_call.join(", ")
        ));
    }
}
