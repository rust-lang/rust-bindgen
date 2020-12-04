use crate::ir::context::GeneratingStage;
use crate::ir::function::FunctionSig;
use crate::ir::item::ItemCanonicalName;
use crate::BindgenContext;
use crate::Item;

pub fn cpp_function_wrapper(
    name: &str,
    ctx: &BindgenContext,
    item: Option<&Item>,
    signature: &FunctionSig,
    canonical_name: &String,
    cpp_out: &mut String,
) {
    assert!(ctx.generating_stage() == GeneratingStage::GeneratingCpp); //todo : should be a debug assert
    let rettype = ctx
        .resolve_item(signature.return_type())
        .expect_type()
        .name();
    if let Some(rettype) = rettype {
        if !canonical_name.starts_with("__") && rettype != "void" {
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
}
