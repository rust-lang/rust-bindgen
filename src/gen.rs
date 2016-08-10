use std;
use hacks::refcell::RefCell;
use std::vec::Vec;
use std::rc::Rc;
use std::collections::HashMap;
use syntax::abi::Abi;
use syntax::ast;
use syntax::codemap::{Span, respan, ExpnInfo, NameAndSpan, MacroBang};
use syntax::ext::base;
use syntax::ext::build::AstBuilder;
use syntax::ext::expand::ExpansionConfig;
use syntax::ext::quote::rt::ToTokens;
use syntax::feature_gate::Features;
use syntax::parse;
use syntax::parse::token::{InternedString, intern};
use syntax::attr::mk_attr_id;
use syntax::ptr::P;
use syntax::print::pprust::tts_to_string;

use super::BindgenOptions;
use super::LinkType;
use parser::Accessor;
use types::*;
use aster;

struct GenCtx<'r> {
    ext_cx: base::ExtCtxt<'r>,
    options: BindgenOptions,
    span: Span,
    module_map: ModuleMap,
    current_module_id: ModuleId,
    saw_union: bool,
}

impl<'r> GenCtx<'r> {
    fn full_path_for_module(&self, id: ModuleId) -> Vec<String> {
        if !self.options.enable_cxx_namespaces {
            return vec![];
        }

        let mut ret = vec![];

        let mut current_id = Some(id);
        while let Some(current) = current_id {
            let module = &self.module_map.get(&current).unwrap();
            ret.push(module.name.clone());
            current_id = module.parent_id;
        }

        if self.current_module_id == ROOT_MODULE_ID {
            ret.pop(); // The root module doens'n need a root:: in the pattern
        }

        ret.reverse();
        ret
    }

    fn current_module_mut(&mut self) -> &mut Module {
        let id = self.current_module_id;
        self.module_map.get_mut(&id).expect("Module not found!")
    }
}

fn first<A, B>((val, _): (A, B)) -> A {
    val
}

fn ref_eq<T>(thing: &T, other: &T) -> bool {
    (thing as *const T) == (other as *const T)
}

fn rust_id(ctx: &mut GenCtx, name: &str) -> (String, bool) {
    let token = parse::token::Ident(ctx.ext_cx.ident_of(name));
    if token.is_any_keyword() ||
        name.contains("@") ||
        name.contains("?") ||
        name.contains("$") ||
        "bool" == name
    {
        let mut s = name.to_owned();
        s = s.replace("@", "_");
        s = s.replace("?", "_");
        s = s.replace("$", "_");
        s.push_str("_");
        (s, true)
    } else {
        (name.to_owned(), false)
    }
}

fn rust_type_id(ctx: &mut GenCtx, name: &str) -> String {
    match name {
        "bool" | "uint" | "u8" | "u16" |
        "u32" | "f32" | "f64" | "i8" |
        "i16" | "i32" | "i64" | "Self" |
        "str" => {
            let mut s = name.to_owned();
            s.push_str("_");
            s
        }
        "int8_t" => "i8".to_owned(),
        "uint8_t" => "u8".to_owned(),
        "int16_t" => "i16".to_owned(),
        "uint16_t" => "u16".to_owned(),
        "int32_t" => "i32".to_owned(),
        "uint32_t" => "u32".to_owned(),
        "int64_t" => "i64".to_owned(),
        "uint64_t" => "u64".to_owned(),
        "uintptr_t"
        | "size_t" => "usize".to_owned(),
        "intptr_t"
        | "ptrdiff_t"
        | "ssize_t" => "isize".to_owned(),
        _ => first(rust_id(ctx, name))
    }
}

fn comp_name(ctx: &GenCtx, kind: CompKind, name: &str) -> String {
    match kind {
        CompKind::Struct => struct_name(ctx, name),
        CompKind::Union  => union_name(ctx, name),
    }
}

fn struct_name(ctx: &GenCtx, name: &str) -> String {
    if ctx.options.rename_types {
        format!("Struct_{}", name)
    } else {
        name.to_owned()
    }
}

fn union_name(ctx: &GenCtx, name: &str) -> String {
    if ctx.options.rename_types {
        format!("Union_{}", name)
    } else {
        name.to_owned()
    }
}

fn enum_name(ctx: &GenCtx, name: &str) -> String {
    if ctx.options.rename_types {
        format!("Enum_{}", name)
    } else {
        name.to_owned()
    }
}

fn gen_unmangle_method(ctx: &mut GenCtx,
                       v: &VarInfo,
                       counts: &mut HashMap<String, isize>,
                       self_kind: Option<ast::Mutability>)
                       -> ast::ImplItem {
    let mut fndecl;
    let mut args = vec![];

    if let Some(mutability) = self_kind {
        let selfexpr = match mutability {
            ast::Mutability::Immutable => quote_expr!(&ctx.ext_cx, &*self),
            ast::Mutability::Mutable => quote_expr!(&ctx.ext_cx, &mut *self),
        };
        args.push(selfexpr);
    }

    match v.ty {
        TFuncPtr(ref sig) => {
            fndecl = cfuncty_to_rs(ctx,
                                   &*sig.ret_ty, sig.args.as_slice(),
                                   false);
            let mut unnamed: usize = 0;
            let iter = if !args.is_empty() {
                sig.args[1..].iter()
            } else {
                sig.args.iter()
            };
            for &(ref n, _) in iter {
                let argname = if n.is_empty() {
                    unnamed += 1;
                    format!("arg{}", unnamed)
                } else {
                    first(rust_id(ctx, &n))
                };
                let expr = aster::AstBuilder::new().expr().path()
                                  .segment(&argname).build().build();
                args.push(expr);
            }
        },
        _ => unreachable!()
    };


    if let Some(mutability) = self_kind {
        assert!(!fndecl.inputs.is_empty());
        fndecl.inputs[0] = ast::Arg {
            ty: P(ast::Ty {
                id: ast::DUMMY_NODE_ID,
                node: ast::TyKind::Rptr(None, ast::MutTy {
                    ty: P(ast::Ty {
                        id: ast::DUMMY_NODE_ID,
                        node: ast::TyKind::ImplicitSelf,
                        span: ctx.span
                    }),
                    mutbl: mutability,
                }),
                span: ctx.span,
            }),
            pat: P(ast::Pat {
                id: ast::DUMMY_NODE_ID,
                node: ast::PatKind::Ident(ast::BindingMode::ByValue(ast::Mutability::Immutable),
                                          respan(ctx.span, ctx.ext_cx.ident_of("self")),
                                          None),
                span: ctx.span,
            }),
            id: ast::DUMMY_NODE_ID,
        };
    }

    let sig = ast::MethodSig {
        unsafety: ast::Unsafety::Unsafe,
        abi: Abi::Rust,
        decl: P(fndecl),
        generics: ast::Generics::default(),
        constness: ast::Constness::NotConst,
    };

    let mangled_rs = first(rust_id(ctx, &v.mangled));
    let call = P(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        node: ast::ExprKind::Call(
            P(ast::Expr {
                id: ast::DUMMY_NODE_ID,
                node: ast::ExprKind::Path(None, ast::Path {
                    span: ctx.span,
                    global: false,
                    segments: vec![ast::PathSegment {
                        identifier: ctx.ext_cx.ident_of(&mangled_rs),
                        parameters: ast::PathParameters::none()
                    }]
                }),
                span: ctx.span,
                attrs: ast::ThinVec::new(),
            }),
            args
        ),
        span: ctx.span,
        attrs: ast::ThinVec::new(),
    });

    let block = ast::Block {
        stmts: vec![
            ast::Stmt {
                id: ast::DUMMY_NODE_ID,
                node: ast::StmtKind::Expr(call),
                span: ctx.span,
            }
        ],
        id: ast::DUMMY_NODE_ID,
        rules: ast::BlockCheckMode::Default,
        span: ctx.span
    };

    let mut name = v.name.clone();
    let mut count = 0;
    match counts.get(&v.name) {
        Some(x) => {
            count = *x;
            name.push_str(&x.to_string());
        },
        None => ()
    }
    count += 1;
    counts.insert(v.name.clone(), count);

    let mut attrs = mk_doc_attr(ctx, &v.comment);
    attrs.push(respan(ctx.span, ast::Attribute_ {
        id: mk_attr_id(),
        style: ast::AttrStyle::Outer,
        value: P(respan(ctx.span, ast::MetaItemKind::Word(InternedString::new("inline")))),
        is_sugared_doc: false
    }));

    let name = first(rust_id(ctx, &name));

    ast::ImplItem {
        id: ast::DUMMY_NODE_ID,
        ident: ctx.ext_cx.ident_of(&name),
        vis: ast::Visibility::Public,
        attrs: attrs,
        node: ast::ImplItemKind::Method(sig, P(block)),
        defaultness: ast::Defaultness::Final,
        span: ctx.span
    }
}

pub fn gen_mods(links: &[(String, LinkType)],
                map: ModuleMap,
                options: BindgenOptions,
                span: Span) -> Vec<P<ast::Item>> {
    // Create a dummy ExtCtxt. We only need this for string interning and that uses TLS.
    let mut features = Features::new();
    features.quote = true;

    let cfg = ExpansionConfig::default("xxx".to_owned());
    let sess = parse::ParseSess::new();
    let mut loader = base::DummyMacroLoader;
    let mut ctx = GenCtx {
        ext_cx: base::ExtCtxt::new(&sess, vec![], cfg, &mut loader),
        options: options,
        span: span,
        module_map: map,
        current_module_id: ROOT_MODULE_ID,
        saw_union: false,
    };

    ctx.ext_cx.bt_push(ExpnInfo {
        call_site: ctx.span,
        callee: NameAndSpan {
            format: MacroBang(intern("")),
            allow_internal_unstable: false,
            span: None
        }
    });

    if let Some(root_mod) = gen_mod(&mut ctx, ROOT_MODULE_ID, links, span) {
        // Move out of the pointer so we can mutate it
        let mut root_mod_item = root_mod.and_then(|item| item);

        gen_union_field_definitions_if_necessary(&mut ctx, &mut root_mod_item);

        if !ctx.options.enable_cxx_namespaces {
            match root_mod_item.node {
                // XXX This clone might be really expensive, but doing:
                ast::ItemKind::Mod(root) => {
                    return root.items;
                }
                _ => unreachable!(),
            }
        }

        let ident = root_mod_item.ident;
        let root_export = quote_item!(&ctx.ext_cx, pub use $ident::*;).unwrap();

        vec![root_export, P(root_mod_item)]
    } else {
        vec![]
    }
}

fn gen_mod(mut ctx: &mut GenCtx,
           module_id: ModuleId,
           links: &[(String, LinkType)],
           span: Span) -> Option<P<ast::Item>> {

    // XXX avoid this clone
    let module = ctx.module_map.get(&module_id).unwrap().clone();

    // Import just the root to minimise name conflicts
    let mut globals = if module_id != ROOT_MODULE_ID {
        // XXX Pass this previously instead of looking it up always?
        let root_ident = ctx.ext_cx.ident_of(&ctx.module_map.get(&ROOT_MODULE_ID).unwrap().name);
        let root_use = quote_item!(&ctx.ext_cx, use $root_ident;).unwrap();
        vec![root_use]
    } else {
        vec![]
    };

    ctx.current_module_id = module_id;

    globals.extend(gen_globals(&mut ctx, links, &module.globals).into_iter());

    globals.extend(module.children_ids.iter().filter_map(|id| {
        gen_mod(ctx, *id, links, span.clone())
    }));

    if !globals.is_empty() {
        Some(P(ast::Item {
            ident: ctx.ext_cx.ident_of(&module.name),
            attrs: vec![],
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemKind::Mod(ast::Mod {
                inner: span,
                items: globals,
            }),
            vis: ast::Visibility::Public,
            span: span.clone(),
        }))
    } else {
        None
    }
}

fn gen_global(mut ctx: &mut GenCtx,
              g: Global,
              defs: &mut Vec<P<ast::Item>>) {
    match g {
        GType(ti) => {
            let t = ti.borrow().clone();
            defs.extend(ctypedef_to_rs(&mut ctx, t).into_iter())
        },
        GCompDecl(ci) => {
            let mut c = ci.borrow().clone();
            let name = comp_name(&ctx, c.kind, &c.name);
            // Use the reference template if any
            while let Some(TComp(ref_template)) = c.ref_template.clone() {
                if c.name != ref_template.borrow().name {
                    break;
                }
                c = ref_template.borrow().clone();
            }
            if !c.args.is_empty() &&
               !c.args.iter().any(|a| a.name().map(|name| name.is_empty()).unwrap_or(true)) {
                defs.extend(comp_to_rs(&mut ctx, &name, c).into_iter());
            } else {
                defs.push(opaque_to_rs(&mut ctx, &name, c.layout));
            }
        },
        GComp(ci) => {
            let c = ci.borrow().clone();
            let name = comp_name(&ctx, c.kind, &c.name);
            defs.extend(comp_to_rs(&mut ctx, &name, c).into_iter())
        },
        GEnumDecl(ei) => {
            let e = ei.borrow().clone();
            let name = enum_name(&ctx, &e.name);
            let dummy = EnumItem::new("_BindgenOpaqueEnum".to_owned(), "".to_owned(), 0);

            defs.extend(cenum_to_rs(&mut ctx, &name, e.kind, e.comment, &[dummy], e.layout).into_iter())
        },
        GEnum(ei) => {
            let e = ei.borrow().clone();
            let name = enum_name(&ctx, &e.name);
            defs.extend(cenum_to_rs(&mut ctx, &name, e.kind, e.comment, &e.items, e.layout).into_iter())
        },
        GVar(vi) => {
            let v = vi.borrow();
            let ty = cty_to_rs(&mut ctx, &v.ty, v.is_const, true);
            defs.push(const_to_rs(&mut ctx, &v.name, v.val.unwrap(), ty));
        },
        _ => { }
    }
}

fn gen_globals(mut ctx: &mut GenCtx,
               links: &[(String, LinkType)],
               globs: &[Global]) -> Vec<P<ast::Item>> {
    let uniq_globs = tag_dup_decl(globs);

    let mut fs = vec![];
    let mut vs = vec![];
    let mut gs = vec![];
    for g in uniq_globs.into_iter() {
        match g {
            GOther => {}
            GFunc(_) => fs.push(g),
            GVar(_) => {
                let is_int_const = {
                    match g {
                        GVar(ref vi) => {
                            let v = vi.borrow();
                            v.is_const && v.val.is_some()
                        }
                        _ => unreachable!()
                    }
                };
                if is_int_const {
                    gs.push(g);
                } else {
                    vs.push(g);
                }
            }
            _ => gs.push(g)
        }
    }

    let mut defs = vec![];
    gs = remove_redundant_decl(gs);

    for mut g in gs.into_iter() {
        if let Some(substituted) = ctx.current_module_mut().translations.remove(&g.name()) {
            match (substituted.layout(), g.layout()) {
                (Some(l), Some(lg)) if l.size == lg.size => {},
                (None, None) => {},
                _ => {
                    warn!("warning: substituted type for {} does not match its size", g.name());
                }
            }
            g = substituted;
        }

        gen_global(ctx, g, &mut defs);
    }

    let mut pending_translations = std::mem::replace(&mut ctx.current_module_mut().translations, HashMap::new());
    for (name, g) in pending_translations.drain() {
        warn!("warning: generating definition for not found type: {}", name);
        gen_global(ctx, g, &mut defs);
    }

    let vars: Vec<_> = vs.into_iter().map(|v| {
        match v {
            GVar(vi) => {
                let v = vi.borrow();
                cvar_to_rs(&mut ctx, v.name.clone(), v.mangled.clone(), &v.ty, v.is_const)
            },
            _ => unreachable!()
        }
    }).collect();

    let mut unmangle_count: HashMap<String, isize> = HashMap::new();
    let funcs = {
        let func_list = fs.into_iter().map(|f| {
            match f {
                GFunc(vi) => {
                    let v = vi.borrow();
                    match v.ty {
                        TFuncPtr(ref sig) => {
                            let mut name = v.name.clone();
                            let mut count = 0;
                            match unmangle_count.get(&v.name) {
                                Some(x) => {
                                    count = *x;
                                    name.push_str(&x.to_string());
                                },
                                None => ()
                            }
                            count += 1;
                            unmangle_count.insert(v.name.clone(), count);

                            let decl = cfunc_to_rs(&mut ctx, name, v.mangled.clone(), v.comment.clone(),
                                                   &*sig.ret_ty, &sig.args[..],
                                                   sig.is_variadic, ast::Visibility::Public);
                            (sig.abi, decl)
                        }
                        _ => unreachable!()
                    }
                },
                _ => unreachable!()
            }
        });

        let mut map: HashMap<Abi, Vec<_>> = HashMap::new();
        for (abi, func) in func_list {
            map.entry(abi).or_insert(vec![]).push(func);
        }
        map
    };

    if !vars.is_empty() {
        defs.push(mk_extern(&mut ctx, links, vars, Abi::C));
    }

    for (abi, funcs) in funcs.into_iter() {
        defs.push(mk_extern(&mut ctx, &links, funcs, abi));
    }

    //let attrs = vec!(mk_attr_list(&mut ctx, "allow", ["dead_code", "non_camel_case_types", "uppercase_variables"]));

    defs
}

fn mk_extern(ctx: &mut GenCtx, links: &[(String, LinkType)],
             foreign_items: Vec<ast::ForeignItem>,
             abi: Abi) -> P<ast::Item> {
    let attrs: Vec<_> = links.iter().map(|&(ref l, ref k)| {
        let k = match *k {
            LinkType::Default => None,
            LinkType::Static => Some("static"),
            LinkType::Framework => Some("framework")
        };
        let link_name = P(respan(ctx.span, ast::MetaItemKind::NameValue(
            InternedString::new("name"),
            respan(ctx.span, ast::LitKind::Str(intern(l).as_str(), ast::StrStyle::Cooked))
        )));
        let link_args = match k {
            None => vec!(link_name),
            Some(ref k) => vec!(link_name, P(respan(ctx.span, ast::MetaItemKind::NameValue(
                InternedString::new("kind"),
                respan(ctx.span, ast::LitKind::Str(intern(k).as_str(), ast::StrStyle::Cooked))
            ))))
        };
        respan(ctx.span, ast::Attribute_ {
            id: mk_attr_id(),
            style: ast::AttrStyle::Outer,
            value: P(respan(ctx.span, ast::MetaItemKind::List(
                InternedString::new("link"),
                link_args)
            )),
            is_sugared_doc: false
        })
    }).collect();

    let mut items = vec![];
    items.extend(foreign_items.into_iter());
    let ext = ast::ItemKind::ForeignMod(ast::ForeignMod {
        abi: abi,
        items: items
    });

    P(ast::Item {
        ident: ctx.ext_cx.ident_of(""),
        attrs: attrs,
        id: ast::DUMMY_NODE_ID,
        node: ext,
        vis: ast::Visibility::Inherited,
        span: ctx.span
    })
}

fn mk_impl(_ctx: &mut GenCtx, ty: P<ast::Ty>,
           items: Vec<ast::ImplItem>)
           -> P<ast::Item> {
    aster::AstBuilder::new().item().impl_().with_items(items).build_ty(ty)
}

fn remove_redundant_decl(gs: Vec<Global>) -> Vec<Global> {
    fn check_decl(a: &Global, ty: &Type) -> bool {
        match *a {
          GComp(ref ci1) => match *ty {
              TComp(ref ci2) => {
                  ref_eq(ci1, ci2) && ci1.borrow().name.is_empty()
              },
              _ => false
          },
          GEnum(ref ei1) => match *ty {
              TEnum(ref ei2) => {
                  ref_eq(ei1, ei2) && ei1.borrow().name.is_empty()
              },
              _ => false
          },
          _ => false
        }
    }

    let typedefs: Vec<Type> = gs.iter().filter_map(|g|
        match *g {
            GType(ref ti) => Some(ti.borrow().ty.clone()),
            _ => None
        }
    ).collect();

    gs.into_iter().filter(|g|
        !typedefs.iter().any(|t| check_decl(g, t))
    ).collect()
}

fn tag_dup_decl(gs: &[Global]) -> Vec<Global> {
    fn check(name1: &str, name2: &str) -> bool {
        !name1.is_empty() && name1 == name2
    }

    fn check_dup(g1: &Global, g2: &Global) -> bool {
        match (g1, g2) {
          (&GType(ref ti1), &GType(ref ti2)) => {
              let a = ti1.borrow();
              let b = ti2.borrow();
              check(&a.name, &b.name)
          },
          (&GComp(ref ci1), &GComp(ref ci2)) => {
              let a = ci1.borrow();
              let b = ci2.borrow();
              check(&a.name, &b.name)
          },
          (&GCompDecl(ref ci1), &GCompDecl(ref ci2)) => {
              let a = ci1.borrow();
              let b = ci2.borrow();
              check(&a.name, &b.name)
          },
          (&GEnum(ref ei1), &GEnum(ref ei2)) => {
              let a = ei1.borrow();
              let b = ei2.borrow();
              check(&a.name, &b.name)
          },
          (&GEnumDecl(ref ei1), &GEnumDecl(ref ei2)) => {
              let a = ei1.borrow();
              let b = ei2.borrow();
              check(&a.name, &b.name)
          },
          (&GVar(ref vi1), &GVar(ref vi2)) => {
              let a = vi1.borrow();
              let b = vi2.borrow();
              check(&a.name, &b.name) &&
              check(&a.mangled, &b.mangled)
          },
          (&GFunc(ref vi1), &GFunc(ref vi2)) => {
              let a = vi1.borrow();
              let b = vi2.borrow();
              check(&a.name, &b.name) &&
              check(&a.mangled, &b.mangled)
          },
          _ => false
        }
    }

    fn check_opaque_dup(g1: &Global, g2: &Global) -> bool {
        match (g1, g2) {
            (&GCompDecl(ref ci1), &GComp(ref ci2)) => {
                let a = ci1.borrow();
                let b = ci2.borrow();
                check(&a.name, &b.name)
            },
            (&GEnumDecl(ref ei1), &GEnum(ref ei2)) => {
                let a = ei1.borrow();
                let b = ei2.borrow();
                check(&a.name, &b.name)
            },
            _ => false,
        }
    }

    if gs.is_empty() {
        return vec![];
    }

    let mut step: Vec<Global> = vec![];
    step.push(gs[0].clone());

    for (i, _gsi) in gs.iter().enumerate().skip(1) {
        let mut dup = false;
        for j in 0..i {
            if i == j {
                continue;
            }
            if check_dup(&gs[i], &gs[j]) {
                dup = true;
                break;
            }
        }
        if !dup {
            step.push(gs[i].clone());
        }
    }

    let len = step.len();
    let mut res: Vec<Global> = vec![];
    for i in 0..len {
        let mut dup = false;
        match &step[i] {
            &GCompDecl(_) | &GEnumDecl(_) => {
                for j in 0..len {
                    if i == j {
                        continue;
                    }
                    if check_opaque_dup(&step[i], &step[j]) {
                        dup = true;
                        break;
                    }
                }
            },
            _ => (),
        }

        if !dup {
            res.push(step[i].clone());
        }
    }

    res
}

fn ctypedef_to_rs(ctx: &mut GenCtx, ty: TypeInfo) -> Vec<P<ast::Item>> {
    fn mk_item(ctx: &mut GenCtx, name: &str, comment: &str, ty: &Type) -> P<ast::Item> {
        let rust_name = rust_type_id(ctx, name);
        let rust_ty = if cty_is_translatable(ty) {
            cty_to_rs(ctx, ty, true, true)
        } else {
            cty_to_rs(ctx, &TVoid, true, true)
        };
        aster::AstBuilder::new().item().pub_()
            .with_attrs(mk_doc_attr(ctx, comment))
            .type_(&rust_name).build_ty(P(rust_ty))
    }

    if ty.opaque {
        return mk_opaque_struct(ctx, &ty.name, &ty.layout);
    }

    let item = match ty.ty {
        TComp(ref ci) => {
            assert!(!ci.borrow().name.is_empty());
            mk_item(ctx, &ty.name, &ty.comment, &ty.ty)
        },
        TEnum(ref ei) => {
            assert!(!ei.borrow().name.is_empty());
            mk_item(ctx, &ty.name, &ty.comment, &ty.ty)
        },
        _ => mk_item(ctx, &ty.name, &ty.comment, &ty.ty),
    };

    vec![item]
}

fn comp_to_rs(ctx: &mut GenCtx, name: &str, ci: CompInfo)
              -> Vec<P<ast::Item>> {
    if ci.hide {
        return vec![];
    }

    if ci.opaque {
        let name = first(rust_id(ctx, &ci.name));
        return mk_opaque_struct(ctx, &name, &ci.layout);
    }

    match ci.kind {
        CompKind::Struct => cstruct_to_rs(ctx, name, ci),
        CompKind::Union => cunion_to_rs(ctx, name, ci),
    }
}

fn comp_attrs(ctx: &GenCtx, ci: &CompInfo, name: &str, extra: &mut Vec<P<ast::Item>>) -> Vec<ast::Attribute> {
    let mut attrs = mk_doc_attr(ctx, &ci.comment);
    attrs.push(mk_repr_attr(ctx, &ci.layout));
    let mut derives = vec![];

    if ci.can_derive_debug() && ctx.options.derive_debug {
        derives.push("Debug");
    }

    if ci.has_destructor() {
        for attr in ctx.options.dtor_attrs.iter() {
            let attr = ctx.ext_cx.ident_of(attr);
            attrs.push(quote_attr!(&ctx.ext_cx, #[$attr]));
        }
    }

    if ci.can_derive_copy() {
        derives.push("Copy");

        // TODO: make mk_clone_impl work for template arguments,
        // meanwhile just fallback to deriving.
        if ci.args.is_empty() {
            extra.push(mk_clone_impl(ctx, name));
        } else {
            derives.push("Clone");
        }
    }

    if !derives.is_empty() {
        attrs.push(mk_deriving_attr(ctx, &derives));
    }

    attrs
}

fn gen_accessors(ctx: &mut GenCtx, name: &str, ty: &ast::Ty, accessor: Accessor,
                 methods: &mut Vec<ast::ImplItem>) {
    if accessor == Accessor::None {
        return;
    }
    let ident =  ctx.ext_cx.ident_of(&format!("{}", name));
    let mutable_getter_name = ctx.ext_cx.ident_of(&format!("get_{}_mut", name));
    let getter_name = ctx.ext_cx.ident_of(&format!("get_{}", name));
    let imp = match accessor {
        Accessor::Regular => quote_item!(&ctx.ext_cx,
            impl X {
                #[inline]
                pub fn $getter_name(&self) -> & $ty {
                    & self.$ident
                }
                pub fn $mutable_getter_name(&mut self) -> &mut $ty {
                    &mut self.$ident
                }
            }
        ),
        Accessor::Unsafe => quote_item!(&ctx.ext_cx,
            impl X {
                #[inline]
                pub unsafe fn $getter_name(&self) -> & $ty {
                    & self.$ident
                }
                pub unsafe fn $mutable_getter_name(&mut self) -> &mut $ty {
                    &mut self.$ident
                }
            }
        ),
        Accessor::Immutable => quote_item!(&ctx.ext_cx,
            impl X {
                #[inline]
                pub fn $getter_name(&self) -> & $ty {
                    & self.$ident
                }
            }
        ),
        _ => return
    };
    match imp.unwrap().node {
        ast::ItemKind::Impl(_, _, _, _, _, ref items) => methods.extend(items.clone()),
        _ => unreachable!()
    }
}

fn cstruct_to_rs(ctx: &mut GenCtx, name: &str, ci: CompInfo) -> Vec<P<ast::Item>> {
    let layout = ci.layout;
    let members = &ci.members;
    let template_args = &ci.args;
    let methodlist = &ci.methods;
    let mut fields = vec![];
    let mut methods = vec![];
    // Nested composites may need to emit declarations and implementations as
    // they are encountered.  The declarations end up in 'extra' and are emitted
    // after the current struct.
    let mut extra = vec![];
    let mut unnamed: u32 = 0;
    let mut bitfields: u32 = 0;

    if ci.has_non_type_template_params ||
       template_args.iter().any(|f| f == &TVoid) {
        return vec![];
    }

    let id = rust_type_id(ctx, name);
    let id_ty = P(mk_ty(ctx, false, &[id.clone()]));

    if ci.has_vtable {
        let mut vffields = vec![];
        let base_vftable = match members.get(0) {
            Some(&CompMember::Field(FieldInfo { ty: TComp(ref ci2), .. })) => {
                let ci2 = ci2.borrow();
                if ci2.has_vtable {
                    Some(format!("_vftable_{}", ci2.name))
                } else {
                    None
                }
            },
            _ => None,
        };

        if let Some(ref base) = base_vftable {
            let field = ast::StructField {
                span: ctx.span,
                vis: ast::Visibility::Public,
                ident: Some(ctx.ext_cx.ident_of("_base")),
                id: ast::DUMMY_NODE_ID,
                ty: P(mk_ty(ctx, false, &[base.clone()])),
                attrs: vec![],
            };
            vffields.push(field);
        }

        for vm in ci.vmethods.iter() {
            let ty = match vm.ty {
                TFuncPtr(ref sig) => {
                    let decl = cfuncty_to_rs(ctx, &*sig.ret_ty, sig.args.as_slice(), sig.is_variadic);
                    mk_fn_proto_ty(ctx, &decl, sig.abi)
                },
                _ => unreachable!()
            };

            let name = first(rust_id(ctx, &vm.name));

            vffields.push(ast::StructField {
                span: ctx.span,
                vis: ast::Visibility::Public,
                ident: Some(ctx.ext_cx.ident_of(&name)),
                id: ast::DUMMY_NODE_ID,
                ty: P(ty),
                attrs: vec![],
            });
        }

        // FIXME: rustc actually generates tons of warnings
        // due to an empty repr(C) type, so we just generate
        // a dummy field with pointer-alignment to supress it.
        if vffields.is_empty() {
            vffields.push(mk_blob_field(ctx, "_bindgen_empty_ctype_warning_fix",
                                        &Layout::new(::std::mem::size_of::<*mut ()>(), ::std::mem::align_of::<*mut ()>())));
        }

        let vf_name = format!("_vftable_{}", name);
        let item = aster::AstBuilder::new().item()
                                           .with_attr(mk_repr_attr(ctx, &layout))
                                           .pub_()
                                           .struct_(&vf_name)
                                           .with_fields(vffields).build();

        extra.push(item);

        if base_vftable.is_none() {
            let vf_type = mk_ty(ctx, false, &[vf_name]);
            fields.push(ast::StructField {
                span: ctx.span,
                ident: Some(ctx.ext_cx.ident_of("_vftable")),
                vis: ast::Visibility::Public,
                id: ast::DUMMY_NODE_ID,
                ty: P(mk_ptrty(ctx, &vf_type, true)),
                attrs: vec![]
            });
        }
    }

    let mut anon_enum_count = 0;
    let mut setters = vec![];
    let mut template_args_used = vec![false; template_args.len()];

    for m in members.iter() {
        match *m {
            CompMember::Enum(ref ei) => {
                let empty_name = ei.borrow().name.is_empty();
                if empty_name {
                    ei.borrow_mut().name = format!("{}_enum{}", name, anon_enum_count);
                    anon_enum_count += 1;
                }

                let e = ei.borrow().clone();
                extra.extend(cenum_to_rs(ctx, &e.name, e.kind, e.comment, &e.items, e.layout).into_iter());
            }
            CompMember::Field(ref f) => {
                let f_name = match f.bitfields {
                    Some(_) => {
                        bitfields += 1;
                        format!("_bitfield_{}", bitfields)
                    }
                    None => rust_type_id(ctx, &f.name)
                };

                let is_translatable = cty_is_translatable(&f.ty);
                if !is_translatable || f.ty.is_opaque() {
                    if !is_translatable {
                        warn!("{}::{} not translatable, void: {}", ci.name, f.name, f.ty == TVoid);
                    }
                    if let Some(layout) = f.ty.layout() {
                        fields.push(mk_blob_field(ctx, &f_name, &layout));
                    }
                    continue;
                }

                if ctx.options.gen_bitfield_methods {
                    let mut offset: u32 = 0;
                    if let Some(ref bitfields) = f.bitfields {
                        for &(ref bf_name, bf_size) in bitfields.iter() {
                            setters.extend(gen_bitfield_methods(ctx, &f_name, bf_name, &f.ty, offset as usize, bf_size).into_iter());
                            offset += bf_size;
                        }
                        setters.push(gen_fullbitfield_method(ctx, &f_name, &f.ty, bitfields))
                    }
                }

                // If the member is not a template argument, it needs the full path.
                let mut needs_full_path = true;
                for (index, arg) in template_args.iter().enumerate() {
                    let used = f.ty.signature_contains_type(arg);

                    if used {
                        template_args_used[index] = true;
                        needs_full_path = *arg == f.ty || match f.ty {
                            TPtr(ref t, _, _, _) => **t != *arg,
                            TArray(ref t, _, _) => **t != *arg,
                            _ => true,
                        };
                        break;
                    }
                }

                let rust_ty = P(cty_to_rs(ctx, &f.ty, f.bitfields.is_none(), needs_full_path));

                // Wrap mutable fields in a Cell/UnsafeCell
                let rust_ty = if f.mutable {
                    if !f.ty.can_derive_copy() {
                        quote_ty!(&ctx.ext_cx, ::std::cell::UnsafeCell<$rust_ty>)
                    // We can only wrap in a cell for non-copiable types, since
                    // Cell<T>: Clone, but not Copy.
                    //
                    // It's fine though, since mutating copiable types is trivial
                    // and doesn't make a lot of sense marking fields as `mutable`.
                    } else if !ci.can_derive_copy() {
                        quote_ty!(&ctx.ext_cx, ::std::cell::Cell<$rust_ty>)
                    } else {
                        rust_ty
                    }
                } else {
                    rust_ty
                };
                let vis = if f.private {
                    ast::Visibility::Inherited
                } else {
                    ast::Visibility::Public
                };
                gen_accessors(ctx, &f_name, &rust_ty, f.accessor, &mut methods);
                let field = ast::StructField {
                    span: ctx.span,
                    ident: Some(ctx.ext_cx.ident_of(&f_name)),
                    vis: vis,
                    id: ast::DUMMY_NODE_ID,
                    ty: rust_ty,
                    attrs: mk_doc_attr(ctx, &f.comment)
                };
                fields.push(field);
            }
            CompMember::Comp(ref rc_c) => {
                let name_is_empty = rc_c.borrow().name.is_empty();

                if name_is_empty {
                    let c = rc_c.borrow();
                    unnamed += 1;
                    let field_name = format!("_bindgen_data_{}_", unnamed);
                    fields.push(mk_blob_field(ctx, &field_name, &c.layout));
                    methods.extend(gen_comp_methods(ctx, &field_name, 0, c.kind, &c.members, &mut extra).into_iter());
                } else {
                    let name = comp_name(&ctx, rc_c.borrow().kind, &rc_c.borrow().name);
                    extra.extend(comp_to_rs(ctx, &name, rc_c.borrow().clone()).into_iter());
                }
            }
        }
    }

    let mut phantom_count = 0;
    for (i, arg) in template_args.iter().enumerate() {
        if template_args_used[i] {
            continue;
        }

        let f_name = format!("_phantom{}", phantom_count);
        phantom_count += 1;
        let inner_type = P(cty_to_rs(ctx, &arg, true, false));

        fields.push(ast::StructField {
            span: ctx.span,
            ident: Some(ctx.ext_cx.ident_of(&f_name)),
            vis: ast::Visibility::Public,
            id: ast::DUMMY_NODE_ID,
            ty: quote_ty!(&ctx.ext_cx, ::std::marker::PhantomData<$inner_type>),
            attrs: vec![],
        });
    }

    if !setters.is_empty() {
        extra.push(mk_impl(ctx, id_ty.clone(), setters));
    }

    let field_count = fields.len();
    let variant_data = if fields.is_empty() {
        ast::VariantData::Unit(ast::DUMMY_NODE_ID)
    } else {
        ast::VariantData::Struct(fields, ast::DUMMY_NODE_ID)
    };

    let ty_params = mk_ty_params(ctx, &template_args);

    let def = ast::ItemKind::Struct(
        variant_data,
        ast::Generics {
            lifetimes: vec![],
            ty_params: P::from_vec(ty_params),
            where_clause: ast::WhereClause {
                id: ast::DUMMY_NODE_ID,
                predicates: vec![]
            }
        }
    );

    let attrs = comp_attrs(&ctx, &ci, name, &mut extra);

    let struct_def = ast::Item {
        ident: ctx.ext_cx.ident_of(&id),
        attrs: attrs,
        id: ast::DUMMY_NODE_ID,
        node: def,
        vis: ast::Visibility::Public,
        span: ctx.span
    };

    let mut items = vec![P(struct_def)];
    if !methods.is_empty() {
        items.push(mk_impl(ctx, id_ty.clone(), methods));
    }

    // Template args have incomplete type in general
    //
    // XXX if x is a class without members, C++ still will report
    // sizeof(x) == 1, since it requires to be adressable.
    //
    // We maybe should add a dummy byte if it's the case, but...
    // That could play wrong with inheritance.
    //
    // So for now don't generate a test if the struct/class is empty
    // or has only empty bases.
    if ci.args.is_empty() && field_count > 0 &&
       (ci.has_nonempty_base || ci.base_members < field_count) {
        extra.push(mk_test_fn(ctx, name, &layout));
    }

    items.extend(extra.into_iter());

    let mut mangledlist = vec![];
    let mut unmangledlist = vec![];
    let mut unmangle_count: HashMap<String, isize> = HashMap::new();
    for v in methodlist {
        let v = v.clone();
        match v.ty {
            TFuncPtr(ref sig) => {
                let name = v.mangled.clone();
                let explicit_self = if v.is_static {
                    None
                } else if v.is_const {
                    Some(ast::Mutability::Immutable)
                } else {
                    Some(ast::Mutability::Mutable)
                };
                unmangledlist.push(gen_unmangle_method(ctx, &v, &mut unmangle_count, explicit_self));
                mangledlist.push(cfunc_to_rs(ctx, name, String::new(), String::new(),
                                             &*sig.ret_ty, sig.args.as_slice(),
                                             sig.is_variadic, ast::Visibility::Inherited));
            }
            _ => unreachable!()
        }
    }
    if !mangledlist.is_empty() {
        items.push(mk_extern(ctx, &[], mangledlist, Abi::C));
        items.push(mk_impl(ctx, id_ty, unmangledlist));
    }

    if !ci.vars.is_empty() && template_args.is_empty() {
        let vars = ci.vars.into_iter().map(|v| {
            let vi = v.varinfo();
            let v = vi.borrow_mut();
            let mut var_name = v.name.clone();
            if !v.mangled.is_empty() {
                var_name = format!("{}_consts_{}", name, v.name);
            }
            cvar_to_rs(ctx, var_name, v.mangled.clone(), &v.ty, v.is_const)
        }).collect();

        items.push(mk_extern(ctx, &[], vars, Abi::C));
    }
    items
}

fn opaque_to_rs(ctx: &mut GenCtx, name: &str, _layout: Layout) -> P<ast::Item> {
    // XXX can't repr(C) an empty enum
    let id = rust_type_id(ctx, name);
    let ident = ctx.ext_cx.ident_of(&id);
    quote_item!(&ctx.ext_cx, pub enum $ident {}).unwrap()
}

fn cunion_to_rs(ctx: &mut GenCtx, name: &str, ci: CompInfo) -> Vec<P<ast::Item>> {
    const UNION_DATA_FIELD_NAME: &'static str = "_bindgen_data_";

    ctx.saw_union = true;

    let members = &ci.members;
    let layout = &ci.layout;

    fn mk_item(ctx: &mut GenCtx, name: &str, item: ast::ItemKind, vis:
               ast::Visibility, attrs: Vec<ast::Attribute>) -> P<ast::Item> {
        P(ast::Item {
            ident: ctx.ext_cx.ident_of(name),
            attrs: attrs,
            id: ast::DUMMY_NODE_ID,
            node: item,
            vis: vis,
            span: ctx.span
        })
    }

    let tmp_ci = Rc::new(RefCell::new(ci.clone()));
    let union = TNamed(Rc::new(RefCell::new(TypeInfo::new(name.to_owned(), ROOT_MODULE_ID, TComp(tmp_ci), layout.clone()))));

    // Nested composites may need to emit declarations and implementations as
    // they are encountered.  The declarations end up in 'extra' and are emitted
    // after the current union.
    let mut extra = vec![];

    fn mk_union_field(ctx: &GenCtx, name: &str, ty: ast::Ty) -> ast::StructField {
        let field_ty = if !ctx.options.enable_cxx_namespaces ||
                          ctx.current_module_id == ROOT_MODULE_ID {
            quote_ty!(&ctx.ext_cx, __BindgenUnionField<$ty>)
        } else {
            quote_ty!(&ctx.ext_cx, root::__BindgenUnionField<$ty>)
        };

        ast::StructField {
            span: ctx.span,
            ident: Some(ctx.ext_cx.ident_of(name)),
            vis: ast::Visibility::Public,
            id: ast::DUMMY_NODE_ID,
            ty: field_ty,
            attrs: vec![],
        }
    }

    let mut fields = members.iter()
                        .flat_map(|member| match *member {
                            CompMember::Field(ref f) => {
                                let cty = cty_to_rs(ctx, &f.ty, false, true);
                                Some(mk_union_field(ctx, &f.name, cty))
                            }
                            _ => None,
                        }).collect::<Vec<_>>();
    fields.push(mk_blob_field(ctx, UNION_DATA_FIELD_NAME, layout));

    // TODO: use aster here.
    let def = ast::ItemKind::Struct(
        ast::VariantData::Struct(fields, ast::DUMMY_NODE_ID),
        ast::Generics::default()
    );

    let union_id = rust_type_id(ctx, name);

    let union_attrs = comp_attrs(&ctx, &ci, name, &mut extra);

    extra.push(mk_test_fn(ctx, &name, &layout));

    let union_def = mk_item(ctx, &union_id, def, ast::Visibility::Public, union_attrs);
    let union_impl = ast::ItemKind::Impl(
        ast::Unsafety::Normal,
        ast::ImplPolarity::Positive,
        ast::Generics::default(),
        None,
        P(cty_to_rs(ctx, &union, true, true)),
        gen_comp_methods(ctx, UNION_DATA_FIELD_NAME, 0, CompKind::Union, &members, &mut extra),
    );
    let mut items = vec!(
        union_def,
        mk_item(ctx, "", union_impl, ast::Visibility::Inherited, vec![])
    );

    items.extend(extra.into_iter());
    items
}

fn const_to_rs(ctx: &mut GenCtx, name: &str, val: i64, val_ty: ast::Ty) -> P<ast::Item> {
    let id = first(rust_id(ctx, name));
    aster::AstBuilder::new().item().pub_().const_(&id)
                            .expr().int(val)
                            .ty().build(P(val_ty))
}

fn enum_size_to_unsigned_max_value(size: usize) -> u64 {
    match size {
        1 => std::u8::MAX as u64,
        2 => std::u16::MAX as u64,
        4 => std::u32::MAX as u64,
        8 => std::u64::MAX,
        _ => unreachable!("invalid enum size: {}", size)
    }
}

fn enum_size_to_rust_type_name(signed: bool, size: usize) -> &'static str {
    match (signed, size) {
        (true, 1) => "i8",
        (false, 1) => "u8",
        (true, 2) => "i16",
        (false, 2) => "u16",
        (true, 4) => "i32",
        (false, 4) => "u32",
        (true, 8) => "i64",
        (false, 8) => "u64",
        _ => {
            warn!("invalid enum decl: signed: {}, size: {}", signed, size);
            "i32"
        }
    }
}

fn cenum_value_to_int_lit(ctx: &mut GenCtx,
                          enum_is_signed: bool,
                          size: usize,
                          value: i64) -> P<ast::Expr> {
    if enum_is_signed {
        if value == std::i64::MIN {
            let lit = ast::LitKind::Int(std::u64::MAX, ast::LitIntType::Unsuffixed);
            ctx.ext_cx.expr_lit(ctx.span, lit)
        } else {
            let lit = ast::LitKind::Int(value.abs() as u64, ast::LitIntType::Unsuffixed);
            let expr = ctx.ext_cx.expr_lit(ctx.span, lit);
            if value < 0 {
                ctx.ext_cx.expr(ctx.span, ast::ExprKind::Unary(ast::UnOp::Neg, expr))
            } else {
                expr
            }
        }
    } else {
        let u64_value = value as u64 & enum_size_to_unsigned_max_value(size);
        let int_lit = ast::LitKind::Int(u64_value, ast::LitIntType::Unsuffixed);
        ctx.ext_cx.expr_lit(ctx.span, int_lit)
    }
}

fn cenum_to_rs(ctx: &mut GenCtx,
               name: &str,
               kind: IKind,
               comment: String,
               enum_items: &[EnumItem],
               layout: Layout) -> Vec<P<ast::Item>> {
    let enum_name = ctx.ext_cx.ident_of(name);
    let enum_ty = ctx.ext_cx.ty_ident(ctx.span, enum_name);
    let enum_is_signed = kind.is_signed();
    let enum_repr = enum_size_to_rust_type_name(enum_is_signed, layout.size);

    let mut items = vec![];

    if !ctx.options.rust_enums {
        items.push(ctx.ext_cx.item_ty(ctx.span,
                                      enum_name,
                                      ctx.ext_cx.ty_ident(ctx.span,
                                                          ctx.ext_cx.ident_of(enum_repr))));
        for item in enum_items {
            let value = cenum_value_to_int_lit(ctx, enum_is_signed, layout.size, item.val);
            let name = first(rust_id(ctx, &item.name));
            items.push(ctx.ext_cx.item_const(ctx.span, ctx.ext_cx.ident_of(&name), enum_ty.clone(), value));
        }
        return items;
    }

    let mut variants = vec![];
    let mut found_values = HashMap::new();
    for item in enum_items {
        let name = first(rust_id(ctx, &item.name));
        let name = ctx.ext_cx.ident_of(&name);
        if let Some(orig) = found_values.get(&item.val) {
            let value = ctx.ext_cx.expr_path(
                ctx.ext_cx.path(ctx.span, vec![enum_name, *orig]));
            items.push(P(ast::Item {
                ident: name,
                attrs: vec![],
                id: ast::DUMMY_NODE_ID,
                node: ast::ItemKind::Const(enum_ty.clone(), value),
                vis: ast::Visibility::Public,
                span: ctx.span,
            }));
            continue;
        }

        found_values.insert(item.val, name);
        let value = cenum_value_to_int_lit(
            ctx, enum_is_signed, layout.size, item.val);

        variants.push(respan(ctx.span, ast::Variant_ {
            name: name,
            attrs: vec![],
            data: ast::VariantData::Unit(ast::DUMMY_NODE_ID),
            disr_expr: Some(value),
        }));
    }

    let enum_repr = InternedString::new(enum_repr);

    let repr_arg = ctx.ext_cx.meta_word(ctx.span, enum_repr);
    let repr_list = ctx.ext_cx.meta_list(ctx.span, InternedString::new("repr"), vec![repr_arg]);

    let mut attrs = mk_doc_attr(ctx, &comment);
    attrs.push(respan(ctx.span, ast::Attribute_ {
        id: mk_attr_id(),
        style: ast::AttrStyle::Outer,
        value: repr_list,
        is_sugared_doc: false,
    }));

    attrs.push(if ctx.options.derive_debug {
        mk_deriving_attr(ctx, &["Debug", "Copy", "Clone", "Eq", "PartialEq", "Hash"])
    } else {
        mk_deriving_attr(ctx, &["Copy", "Clone", "Eq", "PartialEq", "Hash"])
    });

    items.push(P(ast::Item {
        ident: enum_name,
        attrs: attrs,
        id: ast::DUMMY_NODE_ID,
        node: ast::ItemKind::Enum(ast::EnumDef { variants: variants }, ast::Generics::default()),
        vis: ast::Visibility::Public,
        span: ctx.span,
    }));

    items
}

/// Generates accessors for fields in nested structs and unions which must be
/// represented in Rust as an untyped array.  This process may generate
/// declarations and implementations that must be placed at the root level.
/// These are emitted into `extra`.
fn gen_comp_methods(ctx: &mut GenCtx, data_field: &str, data_offset: usize,
                    kind: CompKind, members: &[CompMember],
                    extra: &mut Vec<P<ast::Item>>) -> Vec<ast::ImplItem> {
    let mk_field_method = |ctx: &mut GenCtx, f: &FieldInfo, offset: usize| {
        // TODO: Implement bitfield accessors
        if f.bitfields.is_some() { return None; }

        let f_name = first(rust_id(ctx, &f.name));
        let ret_ty = P(cty_to_rs(ctx, &TPtr(Box::new(f.ty.clone()), false, false, Layout::zero()), true, true));

        // When the offset is zero, generate slightly prettier code.
        let method = {
            let impl_str = format!(r"
                impl X {{
                    pub unsafe fn {}(&mut self) -> {} {{
                        let raw: *mut u8 = ::std::mem::transmute(&self.{});
                        ::std::mem::transmute(raw.offset({}))
                    }}
                }}
            ", f_name, tts_to_string(&ret_ty.to_tokens(&ctx.ext_cx)[..]), data_field, offset);

            parse::new_parser_from_source_str(ctx.ext_cx.parse_sess(),
                ctx.ext_cx.cfg(), "".to_string(), impl_str).parse_item().unwrap().unwrap()
        };

        method.and_then(|i| {
            match i.node {
                ast::ItemKind::Impl(_, _, _, _, _, mut items) => {
                    items.pop()
                }
                _ => unreachable!("impl parsed to something other than impl")
            }
        })
    };

    let mut offset = data_offset;
    let mut methods = vec![];
    for m in members.into_iter() {
        let advance_by = match *m {
            CompMember::Field(ref f) => {
                if ctx.options.gen_bitfield_methods {
                    methods.extend(mk_field_method(ctx, f, offset).into_iter());
                }
                f.ty.size()
            }
            CompMember::Comp(ref rc_c) => {
                let c = rc_c.borrow();
                let name = comp_name(&ctx, c.kind, &c.name);
                extra.extend(comp_to_rs(ctx, &name, c.clone()).into_iter());
                c.layout.size
            }
            CompMember::Enum(_) => 0
        };
        match kind {
            CompKind::Struct => { offset += advance_by; }
            CompKind::Union  => { }
        }
    }
    methods
}

fn type_for_bitfield_width(ctx: &mut GenCtx, width: u32, is_arg: bool) -> ast::Ty {
    let input_type = if width > 16 {
        "u32"
    } else if width > 8 {
        "u16"
    } else if width > 1 {
        "u8"
    } else {
        if is_arg {
            "bool"
        } else {
            "u8"
        }
    };
    mk_ty(ctx, false, &[input_type.to_owned()])
}

fn gen_bitfield_methods(ctx: &mut GenCtx, bindgen_name: &str,
                        field_name: &str, field_type: &Type,
                        offset: usize, width: u32) -> Vec<ast::ImplItem> {
    let input_type = type_for_bitfield_width(ctx, width, true);
    let width = width as usize;

    let field_type = cty_to_rs(ctx, field_type, false, true);

    let real_field_name = if field_name.is_empty() {
        format!("at_offset_{}", offset)
    } else {
        field_name.into()
    };


    let bindgen_ident = ctx.ext_cx.ident_of(bindgen_name);
    let setter_name = ctx.ext_cx.ident_of(&format!("set_{}", real_field_name));
    let getter_name = ctx.ext_cx.ident_of(&real_field_name);

    let mask = ((1usize << width) - 1) << offset;
    let item = quote_item!(&ctx.ext_cx,
        impl X {
            #[inline]
            pub fn $getter_name(&self) -> $field_type {
                (self.$bindgen_ident & ($mask as $field_type)) >> $offset
            }

            #[inline]
            pub fn $setter_name(&mut self, val: $input_type) {
                self.$bindgen_ident &= !($mask as $field_type);
                self.$bindgen_ident |= (val as $field_type << $offset) & ($mask as $field_type);
            }
        }
    ).unwrap();

    match item.node {
        ast::ItemKind::Impl(_, _, _, _, _, ref items) => items.clone(),
        _ => unreachable!()
    }
}

fn gen_fullbitfield_method(ctx: &mut GenCtx, bindgen_name: &String,
                           bitfield_type: &Type, bitfields: &[(String, u32)]) -> ast::ImplItem {
    let field_type = cty_to_rs(ctx, bitfield_type, false, true);
    let mut args = vec![];
    let mut unnamed: usize = 0;
    for &(ref name, width) in bitfields.iter() {
        let ident = if name.is_empty() {
            unnamed += 1;
            let dummy = format!("unnamed_bitfield{}", unnamed);
            ctx.ext_cx.ident_of(&dummy)
        } else {
            ctx.ext_cx.ident_of(name)
        };
        args.push(ast::Arg {
            ty: P(type_for_bitfield_width(ctx, width, true)),
            pat: P(ast::Pat {
                id: ast::DUMMY_NODE_ID,
                node: ast::PatKind::Ident(
                    ast::BindingMode::ByValue(ast::Mutability::Immutable),
                    respan(ctx.span, ident),
                    None
                ),
                span: ctx.span
            }),
            id: ast::DUMMY_NODE_ID,
        });
    }

    let fndecl = ast::FnDecl {
        inputs: args,
        output: ast::FunctionRetTy::Ty(P(field_type.clone())),
        variadic: false
    };

    let mut offset = 0;

    let mut exprs = quote_expr!(&ctx.ext_cx, 0);

    let mut unnamed: usize = 0;
    for &(ref name, width) in bitfields.iter() {
        let name_ident = if name.is_empty() {
            unnamed += 1;
            let dummy = format!("unnamed_bitfield{}", unnamed);
            ctx.ext_cx.ident_of(&dummy)
        } else {
            ctx.ext_cx.ident_of(name)
        };
        exprs = quote_expr!(&ctx.ext_cx,
            $exprs | (($name_ident as $field_type) << $offset)
        );

        offset += width;
    }

    let block = ast::Block {
        stmts: vec![
            ast::Stmt {
                id: ast::DUMMY_NODE_ID,
                node: ast::StmtKind::Expr(exprs),
                span: ctx.span,
            }
        ],
        id: ast::DUMMY_NODE_ID,
        rules: ast::BlockCheckMode::Default,
        span: ctx.span
    };

    let mut attrs = vec![];

    let node = ast::ImplItemKind::Method(
        ast::MethodSig {
            unsafety: ast::Unsafety::Normal,
            abi: Abi::Rust,
            decl: P(fndecl),
            generics: ast::Generics::default(),
            constness: if ctx.options.unstable_rust {
                ast::Constness::Const
            } else {
                attrs.push(quote_attr!(&ctx.ext_cx, #[inline]));
                ast::Constness::NotConst
            },
        }, P(block)
    );

    ast::ImplItem {
        id: ast::DUMMY_NODE_ID,
        ident: ctx.ext_cx.ident_of(&format!("new{}", bindgen_name)),
        vis: ast::Visibility::Public,
        attrs: attrs,
        node: node,
        span: ctx.span,
        defaultness: ast::Defaultness::Final,
    }
}

fn mk_blob_field(ctx: &GenCtx, name: &str, layout: &Layout) -> ast::StructField {
    let ty_name = match layout.align {
        8 => "u64",
        4 => "u32",
        2 => "u16",
        1 | _ => "u8",
    };
    let data_len = if ty_name == "u8" { layout.size } else { layout.size / layout.align };

    let base_ty = mk_ty(ctx, false, &[ty_name.to_owned()]);
    let data_ty = if data_len == 1 {
        P(base_ty)
    } else {
        P(mk_arrty(ctx, &base_ty, data_len))
    };
    ast::StructField {
        span: ctx.span,
        vis: ast::Visibility::Public,
        ident: Some(ctx.ext_cx.ident_of(name)),
        id: ast::DUMMY_NODE_ID,
        ty: data_ty,
        attrs: vec![]
    }
}

fn mk_link_name_attr(ctx: &mut GenCtx, name: String) -> ast::Attribute {
    let lit = respan(ctx.span, ast::LitKind::Str(intern(&name).as_str(), ast::StrStyle::Cooked));
    let attr_val = P(respan(ctx.span, ast::MetaItemKind::NameValue(
        InternedString::new("link_name"), lit
    )));
    let attr = ast::Attribute_ {
        id: mk_attr_id(),
        style: ast::AttrStyle::Outer,
        value: attr_val,
        is_sugared_doc: false
    };
    respan(ctx.span, attr)
}

fn mk_repr_attr(ctx: &GenCtx, layout: &Layout) -> ast::Attribute {
    let mut values = vec!(P(respan(ctx.span, ast::MetaItemKind::Word(InternedString::new("C")))));
    if layout.packed {
        values.push(P(respan(ctx.span, ast::MetaItemKind::Word(InternedString::new("packed")))));
    }
    let attr_val = P(respan(ctx.span, ast::MetaItemKind::List(
        InternedString::new("repr"),
        values
    )));

    respan(ctx.span, ast::Attribute_ {
        id: mk_attr_id(),
        style: ast::AttrStyle::Outer,
        value: attr_val,
        is_sugared_doc: false
    })
}

// NB: This requires that the type you implement it for also
// implements Copy.
//
// Implements std::clone::Clone using dereferencing.
//
// This is to bypass big arrays not implementing clone,
// but implementing copy due to hacks inside rustc's internals.
fn mk_clone_impl(ctx: &GenCtx, ty_name: &str) -> P<ast::Item> {
    let impl_str = format!(r"
        impl ::std::clone::Clone for {} {{
            fn clone(&self) -> Self {{ *self }}
        }}
    ", ty_name);

    parse::new_parser_from_source_str(ctx.ext_cx.parse_sess(),
        ctx.ext_cx.cfg(), "".to_owned(), impl_str).parse_item().unwrap().unwrap()
}

fn mk_deriving_attr(ctx: &GenCtx, attrs: &[&'static str]) -> ast::Attribute {
    let attr_val = P(respan(ctx.span, ast::MetaItemKind::List(
        InternedString::new("derive"),
        attrs.iter().map(|attr| {
            P(respan(ctx.span, ast::MetaItemKind::Word(InternedString::new(attr))))
        }).collect()
    )));

    respan(ctx.span, ast::Attribute_ {
        id: mk_attr_id(),
        style: ast::AttrStyle::Outer,
        value: attr_val,
        is_sugared_doc: false
    })
}

fn mk_doc_attr(ctx: &GenCtx, doc: &str) -> Vec<ast::Attribute> {
    if doc.is_empty() {
        return vec![];
    }

    let attr_val = P(respan(ctx.span, ast::MetaItemKind::NameValue(
        InternedString::new("doc"),
        respan(ctx.span, ast::LitKind::Str(intern(doc).as_str(), ast::StrStyle::Cooked))
    )));

    vec!(respan(ctx.span, ast::Attribute_ {
        id: mk_attr_id(),
        style: ast::AttrStyle::Outer,
        value: attr_val,
        is_sugared_doc: true
    }))
}

fn cvar_to_rs(ctx: &mut GenCtx, name: String,
              mangled: String, ty: &Type,
              is_const: bool) -> ast::ForeignItem {
    let (rust_name, was_mangled) = rust_id(ctx, &name);

    let mut attrs = vec![];
    if !mangled.is_empty() {
        attrs.push(mk_link_name_attr(ctx, mangled));
    } else if was_mangled {
        attrs.push(mk_link_name_attr(ctx, name));
    }

    let val_ty = P(cty_to_rs(ctx, ty, true, true));

    ast::ForeignItem {
        ident: ctx.ext_cx.ident_of(&rust_name),
        attrs: attrs,
        node: ast::ForeignItemKind::Static(val_ty, !is_const),
        id: ast::DUMMY_NODE_ID,
        span: ctx.span,
        vis: ast::Visibility::Public,
    }
}

fn cfuncty_to_rs(ctx: &mut GenCtx,
                 rty: &Type,
                 aty: &[(String, Type)],
                 var: bool) -> ast::FnDecl {

    let ret = match *rty {
        TVoid => ast::FunctionRetTy::Default(ctx.span),
        // Disable references in returns for now
        TPtr(ref t, is_const, _, ref layout) =>
            ast::FunctionRetTy::Ty(P(cty_to_rs(ctx, &TPtr(t.clone(), is_const, false, layout.clone()), true, true))),
        _ => ast::FunctionRetTy::Ty(P(cty_to_rs(ctx, rty, true, true)))
    };

    let mut unnamed: usize = 0;
    let args: Vec<ast::Arg> = aty.iter().map(|arg| {
        let (ref n, ref t) = *arg;

        let arg_name = if n.is_empty() {
            unnamed += 1;
            format!("arg{}", unnamed)
        } else {
            first(rust_id(ctx, &n))
        };

        // From the C90 standard (http://c0x.coding-guidelines.com/6.7.5.3.html)
        // 1598 - A declaration of a parameter as “array of type” shall be
        // adjusted to “qualified pointer to type”, where the type qualifiers
        // (if any) are those specified within the [ and ] of the array type
        // derivation.
        let arg_ty = P(match *t {
            TArray(ref typ, _, l) => cty_to_rs(ctx, &TPtr(typ.clone(), false, false, l), true, true),
            _ => cty_to_rs(ctx, t, true, true),
        });

        ast::Arg {
            ty: arg_ty,
            pat: P(ast::Pat {
                 id: ast::DUMMY_NODE_ID,
                 node: ast::PatKind::Ident(
                     ast::BindingMode::ByValue(ast::Mutability::Immutable),
                     respan(ctx.span, ctx.ext_cx.ident_of(&arg_name)),
                     None
                 ),
                 span: ctx.span
            }),
            id: ast::DUMMY_NODE_ID,
        }
    }).collect();

    let var = !args.is_empty() && var;
    ast::FnDecl {
        inputs: args,
        output: ret,
        variadic: var
    }
}

fn cfunc_to_rs(ctx: &mut GenCtx,
               name: String,
               mangled: String,
               comment: String,
               rty: &Type,
               aty: &[(String, Type)],
               var: bool,
               vis: ast::Visibility) -> ast::ForeignItem {
    let var = !aty.is_empty() && var;
    let decl = ast::ForeignItemKind::Fn(
        P(cfuncty_to_rs(ctx, rty, aty, var)),
        ast::Generics::default()
    );

    let (rust_name, was_mangled) = rust_id(ctx, &name);

    let mut attrs = mk_doc_attr(ctx, &comment);
    if !mangled.is_empty() {
        attrs.push(mk_link_name_attr(ctx, mangled));
    } else if was_mangled {
        attrs.push(mk_link_name_attr(ctx, name));
    }

    ast::ForeignItem {
        ident: ctx.ext_cx.ident_of(&rust_name),
        attrs: attrs,
        node: decl,
        id: ast::DUMMY_NODE_ID,
        span: ctx.span,
        vis: vis,
    }
}

fn cty_to_rs(ctx: &mut GenCtx, ty: &Type, allow_bool: bool, use_full_path: bool) -> ast::Ty {
    let prefix = vec!["std".to_owned(), "os".to_owned(), "raw".to_owned()];
    let raw = |fragment: &str| {
        let mut path = prefix.clone();
        path.push(fragment.to_owned());
        path
    };

    match *ty {
        TVoid => mk_ty(ctx, true, &raw("c_void")),
        TInt(i, ref layout) => match i {
            IBool => {
                let ty_name = match layout.size {
                    1 if allow_bool => "bool",
                    2 => "u16",
                    4 => "u32",
                    8 => "u64",
                    _ => "u8",
                };
                mk_ty(ctx, false, &[ty_name.to_owned()])
            },
            ISChar => mk_ty(ctx, true, &raw("c_char")),
            IUChar => mk_ty(ctx, true, &raw("c_uchar")),
            IInt => mk_ty(ctx, true, &raw("c_int")),
            IUInt => mk_ty(ctx, true, &raw("c_uint")),
            IShort => mk_ty(ctx, true, &raw("c_short")),
            IUShort => mk_ty(ctx, true, &raw("c_ushort")),
            ILong => mk_ty(ctx, true, &raw("c_long")),
            IULong => mk_ty(ctx, true, &raw("c_ulong")),
            ILongLong => mk_ty(ctx, true, &raw("c_longlong")),
            IULongLong => mk_ty(ctx, true, &raw("c_ulonglong"))
        },
        TFloat(f, _) => match f {
            FFloat => mk_ty(ctx, false, &["f32".to_owned()]),
            FDouble => mk_ty(ctx, false, &["f64".to_owned()])
        },
        TPtr(ref t, is_const, _is_ref, _) => {
            let id = cty_to_rs(ctx, &**t, allow_bool, use_full_path);
            mk_ptrty(ctx, &id, is_const)
        },
        TArray(ref t, s, _) => {
            let ty = cty_to_rs(ctx, &**t, allow_bool, use_full_path);
            mk_arrty(ctx, &ty, s)
        },
        TFuncPtr(ref sig) => {
            let decl = cfuncty_to_rs(ctx, &*sig.ret_ty, &sig.args[..], sig.is_variadic);
            mk_fnty(ctx, &decl, sig.abi)
        },
        TFuncProto(ref sig) => {
            let decl = cfuncty_to_rs(ctx, &*sig.ret_ty, &sig.args[..], sig.is_variadic);
            mk_fn_proto_ty(ctx, &decl, sig.abi)
        },
        TNamed(ref ti) => {
            let id = rust_type_id(ctx, &ti.borrow().name);

            if use_full_path {
                let mut path = ctx.full_path_for_module(ti.borrow().module_id);
                path.push(id);
                mk_ty(ctx, false, &path)
            } else {
                mk_ty(ctx, false, &[id])
            }
        },
        TComp(ref ci) => {
            let c = ci.borrow();
            let id = comp_name(&ctx, c.kind, &c.name);

            let args = c.args.iter().map(|gt| {
                P(cty_to_rs(ctx, gt, allow_bool, false))
            }).collect();

            if use_full_path {
                let mut path = ctx.full_path_for_module(c.module_id());
                path.push(id);
                mk_ty_args(ctx, false, &path, args)
            } else {
                mk_ty_args(ctx, false, &[id], args)
            }
        },
        TEnum(ref ei) => {
            let e = ei.borrow();
            let id = enum_name(&ctx, &e.name);

            if use_full_path {
                let mut path = ctx.full_path_for_module(e.module_id);
                path.push(id);
                mk_ty(ctx, false, &path)
            } else {
                mk_ty(ctx, false, &[id])
            }
        }
    }
}

fn cty_is_translatable(ty: &Type) -> bool {
    match *ty {
        TVoid => false,
        TArray(ref t, _, _) => {
            cty_is_translatable(&**t)
        },
        TComp(ref ci) => {
            let c = ci.borrow();
            !c.args.iter().any(|gt| gt == &TVoid) && !c.has_non_type_template_params
        },
        _ => true,
    }
}

fn mk_ty(ctx: &GenCtx, global: bool, segments: &[String]) -> ast::Ty {
    mk_ty_args(ctx, global, segments, vec![])
}

fn mk_ty_args(ctx: &GenCtx, global: bool, segments: &[String], args: Vec<P<ast::Ty>>) -> ast::Ty {
    let segment_count = segments.len();
    let ty = ast::TyKind::Path(
        None,
        ast::Path {
            span: ctx.span,
            global: global,
            segments: segments.iter().enumerate().map(|(i, s)| {
                ast::PathSegment {
                    identifier: ctx.ext_cx.ident_of(s),
                    parameters: ast::PathParameters::AngleBracketed(ast::AngleBracketedParameterData {
                        lifetimes: vec![],
                        types: if i == segment_count - 1 { P::from_vec(args.clone()) } else { P::new() },
                        bindings: P::new(),
                    }),
                }
            }).collect()
        },
    );

    ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: ty,
        span: ctx.span
    }
}

fn mk_ptrty(ctx: &mut GenCtx, base: &ast::Ty, is_const: bool) -> ast::Ty {
    let ty = ast::TyKind::Ptr(ast::MutTy {
        ty: P(base.clone()),
        mutbl: if is_const { ast::Mutability::Immutable } else { ast::Mutability::Mutable }
    });

    ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: ty,
        span: ctx.span
    }
}

#[allow(dead_code)]
fn mk_refty(ctx: &mut GenCtx, base: &ast::Ty, is_const: bool) -> ast::Ty {
    let ty = ast::TyKind::Rptr(
        None,
        ast::MutTy {
            ty: P(base.clone()),
            mutbl: if is_const { ast::Mutability::Immutable } else { ast::Mutability::Mutable }
        }
    );

    ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: ty,
        span: ctx.span
    }
}

fn mk_arrty(ctx: &GenCtx, base: &ast::Ty, n: usize) -> ast::Ty {
    let int_lit = ast::LitKind::Int(n as u64, ast::LitIntType::Unsigned(ast::UintTy::Us));
    let sz = ast::ExprKind::Lit(P(respan(ctx.span, int_lit)));
    let ty = ast::TyKind::FixedLengthVec(
        P(base.clone()),
        P(ast::Expr {
            id: ast::DUMMY_NODE_ID,
            node: sz,
            span: ctx.span,
            attrs: ast::ThinVec::new(),
        })
    );

    ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: ty,
        span: ctx.span
    }
}

fn mk_fn_proto_ty(ctx: &mut GenCtx,
                  decl: &ast::FnDecl,
                  abi: Abi) -> ast::Ty {
    let fnty = ast::TyKind::BareFn(P(ast::BareFnTy {
        unsafety: ast::Unsafety::Unsafe,
        abi: abi,
        lifetimes: vec![],
        decl: P(decl.clone())
    }));

    ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: fnty,
        span: ctx.span,
    }
}

fn mk_fnty(ctx: &mut GenCtx, decl: &ast::FnDecl, abi: Abi) -> ast::Ty {
    let fnty = ast::TyKind::BareFn(P(ast::BareFnTy {
        unsafety: ast::Unsafety::Unsafe,
        abi: abi,
        lifetimes: vec![],
        decl: P(decl.clone())
    }));

    let segs = vec![
        ast::PathSegment {
            identifier: ctx.ext_cx.ident_of("std"),
            parameters: ast::PathParameters::AngleBracketed(ast::AngleBracketedParameterData {
                lifetimes: vec![],
                types: P::new(),
                bindings: P::new(),
            }),
        },
        ast::PathSegment {
            identifier: ctx.ext_cx.ident_of("option"),
            parameters: ast::PathParameters::AngleBracketed(ast::AngleBracketedParameterData {
                lifetimes: vec![],
                types: P::new(),
                bindings: P::new(),
            }),
        },
        ast::PathSegment {
            identifier: ctx.ext_cx.ident_of("Option"),
            parameters: ast::PathParameters::AngleBracketed(ast::AngleBracketedParameterData {
                lifetimes: vec![],
                types: P::from_vec(vec![
                    P(ast::Ty {
                        id: ast::DUMMY_NODE_ID,
                        node: fnty,
                        span: ctx.span
                    })
                ]),
                bindings: P::new(),
            }),
        }
    ];

    ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: ast::TyKind::Path(
            None,
            ast::Path {
                span: ctx.span,
                global: true,
                segments: segs
            },
        ),
        span: ctx.span
    }
}

fn mk_test_fn(ctx: &GenCtx, name: &str, layout: &Layout) -> P<ast::Item> {
    let size = layout.size;
    let align = layout.align;
    let struct_name = ctx.ext_cx.ident_of(name);

    let fn_name = ctx.ext_cx.ident_of(&format!("bindgen_test_layout_{}", name));

    let size_of_expr = quote_expr!(&ctx.ext_cx, ::std::mem::size_of::<$struct_name>());
    let align_of_expr = quote_expr!(&ctx.ext_cx, ::std::mem::align_of::<$struct_name>());
    let item = quote_item!(&ctx.ext_cx,
        #[test]
        fn $fn_name() {
            assert_eq!($size_of_expr, $size);
            assert_eq!($align_of_expr, $align);
        }).unwrap();
    item
}

fn mk_opaque_struct(ctx: &GenCtx, name: &str, layout: &Layout) -> Vec<P<ast::Item>> {
    let blob_field = mk_blob_field(ctx, "_bindgen_opaque_blob", layout);
    let variant_data = if layout.size == 0 {
        ast::VariantData::Unit(ast::DUMMY_NODE_ID)
    } else {
        ast::VariantData::Struct(vec![blob_field], ast::DUMMY_NODE_ID)
    };

    let def = ast::ItemKind::Struct(
        variant_data,
        ast::Generics {
            lifetimes: vec![],
            ty_params: P::new(),
            where_clause: ast::WhereClause {
                id: ast::DUMMY_NODE_ID,
                predicates: vec![]
            }
        }
    );

    let struct_decl = P(ast::Item {
        ident: ctx.ext_cx.ident_of(&name),
        attrs: vec![mk_repr_attr(ctx, layout)],
        id: ast::DUMMY_NODE_ID,
        node: def,
        vis: ast::Visibility::Public,
        span: ctx.span
    });

    let mut ret = vec![struct_decl];

    // The test should always be correct but...
    if *layout != Layout::zero() {
        ret.push(mk_test_fn(ctx, &name, layout));
    }

    ret
}

/// Generates a vector of rust's ty params from a list of types
fn mk_ty_params(ctx: &GenCtx, template_args: &[Type]) -> Vec<ast::TyParam> {
    template_args.iter().map(|gt| {
        let name = match *gt {
            TNamed(ref ti) => {
                ctx.ext_cx.ident_of(&ti.borrow().name)
            },
            _ => ctx.ext_cx.ident_of("")
        };
        ast::TyParam {
            ident: name,
            id: ast::DUMMY_NODE_ID,
            bounds: P::new(),
            default: None,
            span: ctx.span
        }
    }).collect()
}

fn gen_union_field_definitions_if_necessary(ctx: &mut GenCtx, mut root_mod: &mut ast::Item) {
    if !ctx.saw_union {
        return;
    }

    let union_field_decl = quote_item!(&ctx.ext_cx,
        #[derive(Copy, Debug)]
        #[repr(C)]
        pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
    ).unwrap();

    let union_field_impl = quote_item!(&ctx.ext_cx,
        impl<T> __BindgenUnionField<T> {
            #[inline]
            pub fn new() -> Self {
                __BindgenUnionField(::std::marker::PhantomData)
            }

            #[inline]
            pub unsafe fn as_ref(&self) -> &T {
                ::std::mem::transmute(self)
            }

            #[inline]
            pub unsafe fn as_mut(&mut self) -> &mut T {
                ::std::mem::transmute(self)
            }
        }
    ).unwrap();

    let union_field_default_impl = quote_item!(&ctx.ext_cx,
        impl<T> ::std::default::Default for __BindgenUnionField<T> {
            #[inline]
            fn default() -> Self {
                Self::new()
            }
        }
    ).unwrap();

    let union_field_clone_impl = quote_item!(&ctx.ext_cx,
        impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
            #[inline]
            fn clone(&self) -> Self {
                Self::new()
            }
        }
    ).unwrap();

    let items = vec![union_field_decl, union_field_impl, union_field_default_impl, union_field_clone_impl];
    match root_mod.node {
        ast::ItemKind::Mod(ref mut root) => {
            let old_items = std::mem::replace(&mut root.items, items);
            root.items.extend(old_items.into_iter());
        }
        _ => unreachable!(),
    }
}
