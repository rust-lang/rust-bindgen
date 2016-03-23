use std;
use std::cell::RefCell;
use std::vec::Vec;
use std::rc::Rc;
use std::collections::HashMap;
use syntax::abi::Abi;
use syntax::ast;
use syntax::codemap::{Span, Spanned, respan, ExpnInfo, NameAndSpan, MacroBang};
use syntax::ext::base;
use syntax::ext::build::AstBuilder;
use syntax::ext::expand::ExpansionConfig;
use syntax::ext::quote::rt::ToTokens;
use syntax::feature_gate::Features;
use syntax::owned_slice::OwnedSlice;
use syntax::parse;
use syntax::parse::token::{InternedString, intern};
use syntax::attr::mk_attr_id;
use syntax::ptr::P;
use syntax::print::pprust::tts_to_string;

use super::BindgenOptions;
use super::LinkType;
use types::*;

struct GenCtx<'r> {
    ext_cx: base::ExtCtxt<'r>,
    options: BindgenOptions,
    span: Span,
    module_map: ModuleMap,
    current_module_id: ModuleId,
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
}

fn first<A, B>((val, _): (A, B)) -> A {
    val
}

fn ref_eq<T>(thing: &T, other: &T) -> bool {
    (thing as *const T) == (other as *const T)
}

fn empty_generics() -> ast::Generics {
    ast::Generics {
        lifetimes: Vec::new(),
        ty_params: OwnedSlice::empty(),
        where_clause: ast::WhereClause {
            id: ast::DUMMY_NODE_ID,
            predicates: Vec::new()
        }
    }
}

fn rust_id(ctx: &mut GenCtx, name: &str) -> (String, bool) {
    let token = parse::token::Ident(ctx.ext_cx.ident_of(name), parse::token::Plain);
    if token.is_any_keyword() || "bool" == name {
        let mut s = "_".to_string();
        s.push_str(name);
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
            let mut s = "_".to_owned();
            s.push_str(name);
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
        "size_t" => "usize".to_owned(),
        "ssize_t" => "isize".to_owned(),
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
                       self_kind: ast::SelfKind)
                       -> ast::ImplItem {
    let fndecl;
    let mut args = vec!();

    match self_kind {
        ast::SelfKind::Static => (),
        ast::SelfKind::Region(_, mutable, _) => {
            let selfexpr = match mutable {
                ast::Mutability::Immutable => quote_expr!(&ctx.ext_cx, &*self),
                ast::Mutability::Mutable => quote_expr!(&ctx.ext_cx, &mut *self),
            };
            args.push(selfexpr);
        },
        _ => unreachable!()
    }

    match v.ty {
        TFuncPtr(ref sig) => {
            fndecl = cfuncty_to_rs(ctx,
                                   &*sig.ret_ty, sig.args.as_slice(),
                                   false);
            let mut unnamed: usize = 0;
            let iter = if args.len() > 0 {
                sig.args[1..].iter()
            } else {
                sig.args.iter()
            };
            for arg in iter {
                let (ref n, _) = *arg;
                let argname = if n.is_empty() {
                    unnamed += 1;
                    format!("arg{}", unnamed)
                } else {
                    first(rust_id(ctx, &n))
                };
                let expr = ast::Expr {
                    id: ast::DUMMY_NODE_ID,
                    node: ast::ExprKind::Path(None, ast::Path {
                        span: ctx.span,
                        global: false,
                        segments: vec!(ast::PathSegment {
                            identifier: ctx.ext_cx.ident_of(&argname),
                            parameters: ast::PathParameters::none()
                        })
                    }),
                    span: ctx.span,
                    attrs: None,
                };
                args.push(P(expr));
            }
        },
        _ => unreachable!()
    };

    let sig = ast::MethodSig {
        unsafety: ast::Unsafety::Unsafe,
        abi: Abi::Rust,
        decl: P(fndecl),
        generics: empty_generics(),
        explicit_self: respan(ctx.span, self_kind),
        constness: ast::Constness::NotConst,
    };

    let block = ast::Block {
        stmts: vec!(),
        expr: Some(P(ast::Expr {
            id: ast::DUMMY_NODE_ID,
            node: ast::ExprKind::Call(
                P(ast::Expr {
                    id: ast::DUMMY_NODE_ID,
                    node: ast::ExprKind::Path(None, ast::Path {
                        span: ctx.span,
                        global: false,
                        segments: vec!(ast::PathSegment {
                            identifier: ctx.ext_cx.ident_of(&v.mangled),
                            parameters: ast::PathParameters::none()
                        })
                    }),
                    span: ctx.span,
                    attrs: None,
                }),
                args
            ),
            span: ctx.span,
            attrs: None,
        })),
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

    ast::ImplItem {
        id: ast::DUMMY_NODE_ID,
        ident: ctx.ext_cx.ident_of(&name),
        vis: ast::Visibility::Public,
        attrs: attrs,
        node: ast::ImplItemKind::Method(sig, P(block)),
        span: ctx.span
    }
}

pub fn gen_mods(links: &[(String, LinkType)],
                map: ModuleMap,
                options: BindgenOptions,
                span: Span) -> Vec<P<ast::Item>> {
    // Create a dummy ExtCtxt. We only need this for string interning and that uses TLS.
    let mut features = Features::new();
    features.allow_quote = true;
    let cfg = ExpansionConfig {
        crate_name: "xxx".to_owned(),
        features: Some(&features),
        recursion_limit: 64,
        trace_mac: false,
    };
    let sess = &parse::ParseSess::new();
    let mut feature_gated_cfgs = vec![];
    let mut ctx = GenCtx {
        ext_cx: base::ExtCtxt::new(sess, Vec::new(), cfg, &mut feature_gated_cfgs),
        options: options,
        span: span,
        module_map: map,
        current_module_id: ROOT_MODULE_ID,
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
        if !ctx.options.enable_cxx_namespaces {
            match root_mod.node {
                // XXX This clone might be really expensive, but doing:
                // ast::ItemMod(ref mut root) => {
                //     return ::std::mem::replace(&mut root.items, vec![]);
                // }
                // fails with "error: cannot borrow immutable anonymous field as mutable".
                // So...
                ast::ItemKind::Mod(ref root) => {
                    return root.items.clone()
                }
                _ => unreachable!(),
            }
        }

        let root_export = P(ast::Item {
            ident: ctx.ext_cx.ident_of(""),
            attrs: vec![],
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemKind::Use(P(
                Spanned {
                    node: ast::ViewPathGlob(ast::Path {
                        span: span.clone(),
                        global: false,
                        segments: vec![ast::PathSegment {
                            identifier: root_mod.ident,
                            parameters: ast::PathParameters::none(),
                        }]
                    }),
                    span: span.clone(),
                })),
            vis: ast::Visibility::Public,
            span: span.clone(),
        });

        vec![root_export, root_mod]
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
        let root = ctx.ext_cx.ident_of(&ctx.module_map.get(&ROOT_MODULE_ID).unwrap().name);
        vec![P(ast::Item {
            ident: ctx.ext_cx.ident_of(""),
            attrs: vec![],
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemKind::Use(P(
                Spanned {
                    node: ast::ViewPathSimple(root.clone(),
                              ast::Path {
                                  span: span.clone(),
                                  global: false,
                                  segments: vec![ast::PathSegment {
                                      identifier: root,
                                      parameters: ast::PathParameters::none(),
                                  }]
                              }),
                    span: span.clone(),
                })),
            vis: ast::Visibility::Public,
            span: span.clone(),
        })]
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

fn type_blacklisted(ctx: &GenCtx, global: &Global) -> bool {
    let global_name = global.name();

    ctx.options.blacklist_type.iter().all(|name| *name != global_name)
}

fn gen_globals(mut ctx: &mut GenCtx,
               links: &[(String, LinkType)],
               globs: &[Global]) -> Vec<P<ast::Item>> {
    let uniq_globs = tag_dup_decl(globs);

    let mut fs = vec!();
    let mut vs = vec!();
    let mut gs = vec!();
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

    let mut defs = vec!();
    gs = remove_redundant_decl(gs);

    for g in gs.into_iter() {
        if !type_blacklisted(ctx, &g) {
            continue;
        }

        match g {
            GType(ti) => {
                let t = ti.borrow().clone();
                defs.push(ctypedef_to_rs(&mut ctx, t))
            },
            GCompDecl(ci) => {
                let c = ci.borrow().clone();
                let name = comp_name(&ctx, c.kind, &c.name);

                defs.push(opaque_to_rs(&mut ctx, &name));
            },
            GComp(ci) => {
                let c = ci.borrow().clone();
                let name = comp_name(&ctx, c.kind, &c.name);
                defs.extend(comp_to_rs(&mut ctx, &name, c).into_iter())
            },
            GEnumDecl(ei) => {
                let e = ei.borrow().clone();
                let name = enum_name(&ctx, &e.name);

                defs.push(opaque_to_rs(&mut ctx, &name));
            },
            GEnum(ei) => {
                let e = ei.borrow().clone();
                let name = enum_name(&ctx, &e.name);
                defs.extend(cenum_to_rs(&mut ctx, name, e.kind, e.comment, &e.items, e.layout).into_iter())
            },
            GVar(vi) => {
                let v = vi.borrow();
                let ty = cty_to_rs(&mut ctx, &v.ty, v.is_const, true);
                defs.push(const_to_rs(&mut ctx, v.name.clone(), v.val.unwrap(), ty));
            },
            _ => { }
        }
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

    let mut items = Vec::new();
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

fn mk_impl(ctx: &mut GenCtx, ty: P<ast::Ty>,
           items: Vec<ast::ImplItem>)
           -> P<ast::Item> {
    let ext = ast::ItemKind::Impl(
        ast::Unsafety::Normal,
        ast::ImplPolarity::Positive,
        empty_generics(),
        None,
        ty,
        items
    );

    P(ast::Item {
        ident: ctx.ext_cx.ident_of(""),
        attrs: vec!(),
        id: ast::DUMMY_NODE_ID,
        node: ext,
        vis: ast::Visibility::Inherited,
        span: ctx.span
    })
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

    let mut step: Vec<Global> = vec!();
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
    let mut res: Vec<Global> = vec!();
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

fn ctypedef_to_rs(ctx: &mut GenCtx, ty: TypeInfo) -> P<ast::Item> {
    fn mk_item(ctx: &mut GenCtx, name: &str, comment: &str, ty: &Type) -> P<ast::Item> {
        let rust_name = rust_type_id(ctx, name);
        let rust_ty = if cty_is_translatable(ty) {
            cty_to_rs(ctx, ty, true, true)
        } else {
            cty_to_rs(ctx, &TVoid, true, true)
        };
        let base = ast::ItemKind::Ty(
            P(ast::Ty {
                id: ast::DUMMY_NODE_ID,
                node: rust_ty.node,
                span: ctx.span,
            }),
            empty_generics()
        );

        P(ast::Item {
            ident: ctx.ext_cx.ident_of(&rust_name),
            attrs: mk_doc_attr(ctx, comment),
            id: ast::DUMMY_NODE_ID,
            node: base,
            vis: ast::Visibility::Public,
            span: ctx.span
        })
    }

    match ty.ty {
        TComp(ref ci) => {
            assert!(!ci.borrow().name.is_empty());
            mk_item(ctx, &ty.name, &ty.comment, &ty.ty)
        },
        TEnum(ref ei) => {
            assert!(!ei.borrow().name.is_empty());
            mk_item(ctx, &ty.name, &ty.comment, &ty.ty)
        },
        _ => mk_item(ctx, &ty.name, &ty.comment, &ty.ty),
    }
}

fn comp_to_rs(ctx: &mut GenCtx, name: &str, ci: CompInfo)
              -> Vec<P<ast::Item>> {
    match ci.kind {
        CompKind::Struct => cstruct_to_rs(ctx, name, ci),
        CompKind::Union =>  cunion_to_rs(ctx, name, ci.layout, ci.members),
    }
}

fn cstruct_to_rs(ctx: &mut GenCtx, name: &str, ci: CompInfo) -> Vec<P<ast::Item>> {
    let layout = ci.layout;
    let members = &ci.members;
    let template_args = &ci.args;
    let methodlist = &ci.methods;
    let mut fields = vec!();
    let mut methods = vec!();
    // Nested composites may need to emit declarations and implementations as
    // they are encountered.  The declarations end up in 'extra' and are emitted
    // after the current struct.
    let mut extra = vec!();
    let mut unnamed: u32 = 0;
    let mut bitfields: u32 = 0;

    if ci.hide ||
       ci.has_non_type_template_params ||
       template_args.iter().any(|f| f == &TVoid) {
        return vec!();
    }

    let id = rust_type_id(ctx, name);
    let id_ty = P(mk_ty(ctx, false, &[id.clone()]));

    if ci.has_vtable {
        let mut vffields = vec!();
        let base_vftable = if !members.is_empty() {
            if let CompMember::Field(ref fi) = members[0] {
                match fi.ty {
                    TComp(ref ci2) => {
                        let ci2 = ci2.borrow();
                        if ci2.has_vtable {
                            Some(format!("_vftable_{}", ci2.name))
                        } else {
                            None
                        }
                    },
                    _ => None
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some(ref base) = base_vftable {
            let field = ast::StructField_ {
                kind: ast::NamedField(ctx.ext_cx.ident_of("_base"), ast::Visibility::Public),
                id: ast::DUMMY_NODE_ID,
                ty: P(mk_ty(ctx, false, &[base.clone()])),
                attrs: vec!(),
            };
            vffields.push(respan(ctx.span, field));
        }

        for vm in ci.vmethods.iter() {
            let ty = match vm.ty {
                TFuncPtr(ref sig) => {
                    let decl = cfuncty_to_rs(ctx, &*sig.ret_ty, sig.args.as_slice(), sig.is_variadic);
                    mk_fn_proto_ty(ctx, &decl, sig.abi)
                },
                _ => unreachable!()
            };
            let field = ast::StructField_ {
                kind: ast::NamedField(ctx.ext_cx.ident_of(&vm.name), ast::Visibility::Public),
                id: ast::DUMMY_NODE_ID,
                ty: P(ty),
                attrs: vec!(),
            };
            vffields.push(respan(ctx.span, field));
        }

        let vf_name = format!("_vftable_{}", name);
        let item = P(ast::Item {
            ident: ctx.ext_cx.ident_of(&vf_name),
            attrs: vec!(mk_repr_attr(ctx, layout)),
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemKind::Struct(
                ast::VariantData::Struct(vffields, ast::DUMMY_NODE_ID),
                empty_generics()
            ),
            vis: ast::Visibility::Public,
            span: ctx.span,
        });
        extra.push(item);

        if base_vftable.is_none() {
            let vf_type = mk_ty(ctx, false, &[vf_name]);
            fields.push(respan(ctx.span, ast::StructField_ {
                kind: ast::NamedField(ctx.ext_cx.ident_of("_vftable"), ast::Visibility::Public),
                id: ast::DUMMY_NODE_ID,
                ty: P(mk_ptrty(ctx, &vf_type, true)),
                attrs: Vec::new()
            }));
        }
    }

    if members.is_empty() {
        let mut phantom_count = 0;
        for arg in template_args {
            let f_name = format!("_phantom{}", phantom_count);
            phantom_count += 1;
            let inner_type = P(cty_to_rs(ctx, &arg, true, false));
            fields.push(respan(ctx.span, ast::StructField_ {
                kind: ast::NamedField(
                    ctx.ext_cx.ident_of(&f_name),
                    ast::Visibility::Public,
                ),
                id: ast::DUMMY_NODE_ID,
                ty: quote_ty!(&ctx.ext_cx, ::std::marker::PhantomData<$inner_type>),
                attrs: vec!(),
            }));
        }
    }

    let mut anon_enum_count = 0;
    let mut setters = vec!();
    let mut has_destructor = ci.has_destructor;

    for m in members.iter() {
        if let CompMember::Enum(ref ei) = *m {
            let e = ei.borrow().clone();
            let ename = if e.name.is_empty() {
                let ename = format!("{}_enum{}", name, anon_enum_count);
                anon_enum_count += 1;
                ename
            } else {
                e.name.clone()
            };
            extra.extend(cenum_to_rs(ctx, ename, e.kind, e.comment, &e.items, e.layout).into_iter());
            continue;
        }

        fn comp_fields(m: &CompMember)
                       -> (Option<Rc<RefCell<CompInfo>>>, Option<FieldInfo>) {
            match *m {
                CompMember::Field(ref f) => { (None, Some(f.clone())) }
                CompMember::Comp(ref rc_c) => {
                    let c = rc_c.borrow();
                    if c.members.len() == 1 {
                        comp_fields(&c.members[0])
                    } else {
                        (Some(rc_c.clone()), None)
                    }
                }
                CompMember::CompField(ref rc_c, ref f) => { (Some(rc_c.clone()), Some(f.clone())) }
                _ => unreachable!()
            }
        }

        let (opt_rc_c, opt_f) = comp_fields(m);

        if let Some(f) = opt_f {
            if cty_has_destructor(&f.ty) {
                has_destructor = true;
            }

            let f_name = match f.bitfields {
                Some(_) => {
                    bitfields += 1;
                    format!("_bitfield_{}", bitfields)
                }
                None => rust_type_id(ctx, &f.name)
            };

            if !cty_is_translatable(&f.ty) {
                println!("{}::{} not translatable, void: {}", ci.name, f.name, f.ty == TVoid);
                let size = f.ty.size();

                if size != 0 {
                    fields.push(respan(ctx.span, ast::StructField_ {
                        kind: ast::NamedField(
                            ctx.ext_cx.ident_of(&f_name),
                            ast::Visibility::Public,
                        ),
                        id: ast::DUMMY_NODE_ID,
                        ty: quote_ty!(&ctx.ext_cx, [u8; $size]),
                        attrs: mk_doc_attr(ctx, &f.comment)
                    }));
                }
                continue;
            }

            let mut offset: u32 = 0;
            if let Some(ref bitfields) = f.bitfields {
                for &(ref bf_name, bf_size) in bitfields.iter() {
                    setters.push(gen_bitfield_method(ctx, &f_name, bf_name, &f.ty, offset as usize, bf_size));
                    offset += bf_size;
                }
                setters.push(gen_fullbitfield_method(ctx, &f_name, &f.ty, bitfields))
            }

            let mut bypass = false;
            let f_ty = if let Some(ref rc_c) = opt_rc_c {
                if rc_c.borrow().members.len() == 1 {
                    if let CompMember::Field(ref inner_f) = rc_c.borrow().members[0] {
                        bypass = true;
                        inner_f.ty.clone()
                    } else {
                        f.ty.clone()
                    }
                } else {
                    f.ty.clone()
                }
            } else {
                f.ty.clone()
            };

            // If the member is not a template argument, it needs the full path.
            let needs_full_path = !template_args.iter().any(|arg| *arg == f_ty);
            let f_ty = P(cty_to_rs(ctx, &f_ty, f.bitfields.is_none(), needs_full_path));

            fields.push(respan(ctx.span, ast::StructField_ {
                kind: ast::NamedField(
                    ctx.ext_cx.ident_of(&f_name),
                    ast::Visibility::Public,
                ),
                id: ast::DUMMY_NODE_ID,
                ty: f_ty,
                attrs: mk_doc_attr(ctx, &f.comment)
            }));
            if bypass {
                continue;
            }
        }

        if let Some(mut rc_c) = opt_rc_c {
            let name_is_empty = rc_c.borrow().name.is_empty();

            if name_is_empty {
                let c = rc_c.borrow();
                unnamed += 1;
                let field_name = format!("_bindgen_data_{}_", unnamed);
                fields.push(mk_blob_field(ctx, &field_name, c.layout));
                methods.extend(gen_comp_methods(ctx, &field_name, 0, c.kind, &c.members, &mut extra).into_iter());
            } else {
                // Mangled name to deal with multiple definitions of the same inner type
                let mangled_name = [name, &*rc_c.borrow().name].join("_").to_owned();
                rc_c.borrow_mut().name = mangled_name;
                let name = comp_name(&ctx, rc_c.borrow().kind, &rc_c.borrow().name);
                extra.extend(comp_to_rs(ctx, &name, rc_c.borrow().clone()).into_iter());
            }
        }
    }
    if !setters.is_empty() {
        extra.push(P(ast::Item {
            ident: ctx.ext_cx.ident_of(""),
            attrs: vec!(),
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemKind::Impl(
                ast::Unsafety::Normal,
                ast::ImplPolarity::Positive,
                empty_generics(),
                None,
                id_ty.clone(),
                setters
            ),
            vis: ast::Visibility::Inherited,
            span: ctx.span
        }));
    }

    let field_count = fields.len();
    let variant_data = if fields.is_empty() {
        ast::VariantData::Unit(ast::DUMMY_NODE_ID)
    } else {
        ast::VariantData::Struct(fields, ast::DUMMY_NODE_ID)
    };
    let ty_params = template_args.iter().map(|gt| {
        let name = match gt {
            &TNamed(ref ti) => {
                ctx.ext_cx.ident_of(&ti.borrow().name)
            },
            _ => ctx.ext_cx.ident_of("")
        };
        ast::TyParam {
            ident: name,
            id: ast::DUMMY_NODE_ID,
            bounds: OwnedSlice::empty(),
            default: None,
            span: ctx.span
        }
    }).collect();

    let def = ast::ItemKind::Struct(
        variant_data,
        ast::Generics {
            lifetimes: vec!(),
            ty_params: OwnedSlice::from_vec(ty_params),
            where_clause: ast::WhereClause {
                id: ast::DUMMY_NODE_ID,
                predicates: vec!()
            }
        }
    );

    let mut attrs = mk_doc_attr(ctx, &ci.comment);
    attrs.push(mk_repr_attr(ctx, layout));
    if !has_destructor {
        attrs.push(mk_deriving_copy_attr(ctx));
    }
    let struct_def = ast::Item {
        ident: ctx.ext_cx.ident_of(&id),
        attrs: attrs,
        id: ast::DUMMY_NODE_ID,
        node: def,
        vis: ast::Visibility::Public,
        span: ctx.span
    };

    let mut items = vec!(P(struct_def));
    if !methods.is_empty() {
        let impl_ = ast::ItemKind::Impl(
            ast::Unsafety::Normal,
            ast::ImplPolarity::Positive,
            empty_generics(),
            None,
            id_ty.clone(),
            methods
        );
        items.push(
            P(ast::Item {
                ident: ctx.ext_cx.ident_of(&name),
                attrs: vec!(),
                id: ast::DUMMY_NODE_ID,
                node: impl_,
                vis: ast::Visibility::Inherited,
                span: ctx.span}));
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

    let mut mangledlist = vec!();
    let mut unmangledlist = vec!();
    let mut unmangle_count: HashMap<String, isize> = HashMap::new();
    for v in methodlist {
        let v = v.clone();
        match v.ty {
            TFuncPtr(ref sig) => {
                let name = v.mangled.clone();
                let explicit_self = if v.is_static {
                    ast::SelfKind::Static
                } else if v.is_const {
                    ast::SelfKind::Region(None, ast::Mutability::Immutable, ctx.ext_cx.ident_of("self"))
                } else {
                    ast::SelfKind::Region(None, ast::Mutability::Mutable, ctx.ext_cx.ident_of("self"))
                };
                unmangledlist.push(gen_unmangle_method(ctx, &v, &mut unmangle_count, explicit_self));
                mangledlist.push(cfunc_to_rs(ctx, name, String::new(), String::new(),
                                             &*sig.ret_ty, sig.args.as_slice(),
                                             sig.is_variadic, ast::Visibility::Inherited));
            }
            _ => unreachable!()
        }
    }
    if mangledlist.len() > 0 {
        items.push(mk_extern(ctx, &vec!(), mangledlist, Abi::C));
        items.push(mk_impl(ctx, id_ty, unmangledlist));
    }
    items
}

fn opaque_to_rs(ctx: &mut GenCtx, name: &str) -> P<ast::Item> {
    let def = ast::ItemKind::Enum(
        ast::EnumDef {
           variants: vec!()
        },
        empty_generics()
    );

    let id = rust_type_id(ctx, name);
    P(ast::Item {
        ident: ctx.ext_cx.ident_of(&id),
        attrs: Vec::new(),
        id: ast::DUMMY_NODE_ID,
        node: def,
        vis: ast::Visibility::Public,
        span: ctx.span
    })
}

fn cunion_to_rs(ctx: &mut GenCtx, name: &str, layout: Layout, members: Vec<CompMember>) -> Vec<P<ast::Item>> {
    fn mk_item(ctx: &mut GenCtx, name: String, item: ast::ItemKind, vis:
               ast::Visibility, attrs: Vec<ast::Attribute>) -> P<ast::Item> {
        P(ast::Item {
            ident: ctx.ext_cx.ident_of(&name),
            attrs: attrs,
            id: ast::DUMMY_NODE_ID,
            node: item,
            vis: vis,
            span: ctx.span
        })
    }

    // XXX what module id is correct?
    let ci = Rc::new(RefCell::new(CompInfo::new(name.to_owned(), ROOT_MODULE_ID, name.to_owned(), "".to_owned(), CompKind::Union, members.clone(), layout)));
    let union = TNamed(Rc::new(RefCell::new(TypeInfo::new(name.to_owned(), ROOT_MODULE_ID, TComp(ci), layout))));

    // Nested composites may need to emit declarations and implementations as
    // they are encountered.  The declarations end up in 'extra' and are emitted
    // after the current union.
    let mut extra = vec!();

    let data_field_name = "_bindgen_data_";
    let data_field = mk_blob_field(ctx, data_field_name, layout);

    let def = ast::ItemKind::Struct(
        ast::VariantData::Struct(vec![data_field], ast::DUMMY_NODE_ID),
        empty_generics()
    );
    let union_id = rust_type_id(ctx, name);
    let mut union_attrs = vec!(mk_repr_attr(ctx, layout));
    let can_derive_debug = members.iter()
                                  .all(|member| match *member {
                                      CompMember::Field(ref f) |
                                      CompMember::CompField(_, ref f) => f.ty.can_derive_debug(),
                                      _ => true
                                  });
    union_attrs.push(if can_derive_debug {
        mk_deriving_copy_and_maybe_debug_attr(ctx)
    } else {
        mk_deriving_copy_attr(ctx)
    });

    let union_def = mk_item(ctx, union_id, def, ast::Visibility::Public, union_attrs);

    let union_impl = ast::ItemKind::Impl(
        ast::Unsafety::Normal,
        ast::ImplPolarity::Positive,
        empty_generics(),
        None,
        P(cty_to_rs(ctx, &union, true, true)),
        gen_comp_methods(ctx, data_field_name, 0, CompKind::Union, &members, &mut extra),
    );

    let mut items = vec!(
        union_def,
        mk_item(ctx, "".to_owned(), union_impl, ast::Visibility::Inherited, Vec::new())
    );

    items.extend(extra.into_iter());
    items
}

fn const_to_rs(ctx: &mut GenCtx, name: String, val: i64, val_ty: ast::Ty) -> P<ast::Item> {
    let int_lit = ast::LitKind::Int(
        val.abs() as u64,
        ast::LitIntType::Unsuffixed
            );
    let mut value = ctx.ext_cx.expr_lit(ctx.span, int_lit);
    if val < 0 {
        let negated = ast::ExprKind::Unary(ast::UnOp::Neg, value);
        value = ctx.ext_cx.expr(ctx.span, negated);
    }

    let cst = ast::ItemKind::Const(
        P(val_ty),
        value
            );

    let id = first(rust_id(ctx, &name));
    P(ast::Item {
        ident: ctx.ext_cx.ident_of(&id[..]),
        attrs: Vec::new(),
        id: ast::DUMMY_NODE_ID,
        node: cst,
        vis: ast::Visibility::Public,
        span: ctx.span
    })
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
            println!("invalid enum decl: signed: {}, size: {}", signed, size);
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
               name: String,
               kind: IKind,
               comment: String,
               enum_items: &[EnumItem],
               layout: Layout) -> Vec<P<ast::Item>> {
    let enum_name = ctx.ext_cx.ident_of(&name);
    let enum_ty = ctx.ext_cx.ty_ident(ctx.span, enum_name);
    let enum_is_signed = kind.is_signed();
    let enum_repr = enum_size_to_rust_type_name(enum_is_signed, layout.size);

    // Rust is not happy with univariant enums
    // if items.len() < 2 {
    //     return vec!();
    // }
    //
    let mut items = vec![];

    if !ctx.options.rust_enums {
        items.push(ctx.ext_cx.item_ty(ctx.span,
                                      enum_name,
                                      ctx.ext_cx.ty_ident(ctx.span,
                                                          ctx.ext_cx.ident_of(enum_repr))));
        for item in enum_items {
            let value = cenum_value_to_int_lit(ctx, enum_is_signed, layout.size, item.val);
            items.push(ctx.ext_cx.item_const(ctx.span, ctx.ext_cx.ident_of(&item.name), enum_ty.clone(), value));
        }
        return items;
    }

    let mut variants = vec![];
    let mut found_values = HashMap::new();
    for item in enum_items {
        let name = ctx.ext_cx.ident_of(&item.name);
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

    attrs.push(mk_deriving_copy_and_maybe_debug_attr(ctx));

    items.push(P(ast::Item {
        ident: enum_name,
        attrs: attrs,
        id: ast::DUMMY_NODE_ID,
        node: ast::ItemKind::Enum(ast::EnumDef { variants: variants }, empty_generics()),
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
    let mut methods = vec!();
    for m in members.into_iter() {
        let advance_by = match *m {
            CompMember::Field(ref f) => {
                methods.extend(mk_field_method(ctx, f, offset).into_iter());
                f.ty.size()
            }
            CompMember::Comp(ref rc_c) => {
                let c = &rc_c.borrow();
                methods.extend(gen_comp_methods(ctx, data_field, offset, c.kind,
                                                &c.members, extra).into_iter());
                c.layout.size
            }
            CompMember::CompField(ref rc_c, ref f) => {
                methods.extend(mk_field_method(ctx, f, offset).into_iter());

                let c = rc_c.borrow();
                let name = comp_name(&ctx, c.kind, &c.name);
                extra.extend(comp_to_rs(ctx, &name, c.clone()).into_iter());
                f.ty.size()
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

fn type_for_bitfield_width(ctx: &mut GenCtx, width: u32) -> ast::Ty {
    let input_type = if width > 16 {
        "u32"
    } else if width > 8 {
        "u16"
    } else if width > 1 {
        "u8"
    } else {
        "bool"
    };
    mk_ty(ctx, false, &[input_type.to_owned()])
}

fn gen_bitfield_method(ctx: &mut GenCtx, bindgen_name: &String,
                       field_name: &String, field_type: &Type,
                       offset: usize, width: u32) -> ast::ImplItem {
    let input_type = type_for_bitfield_width(ctx, width);
    let field_type = cty_to_rs(ctx, &field_type, false, true);
    let setter_name = ctx.ext_cx.ident_of(&format!("set_{}", field_name));
    let bindgen_ident = ctx.ext_cx.ident_of(&*bindgen_name);

    let node = &quote_item!(&ctx.ext_cx,
        impl X {
            pub fn $setter_name(&mut self, val: $input_type) {
                self.$bindgen_ident &= !(((1 << $width) - 1) << $offset);
                self.$bindgen_ident |= (val as $field_type) << $offset;
            }
        }
    ).unwrap().node;
    match node {
        &ast::ItemKind::Impl(_, _, _, _, _, ref items) => items[0].clone(),
        _ => unreachable!()
    }
}

fn gen_fullbitfield_method(ctx: &mut GenCtx, bindgen_name: &String,
                           field_type: &Type, bitfields: &[(String, u32)]) -> ast::ImplItem {
    let field_type = cty_to_rs(ctx, field_type, false, true);
    let mut args = vec!();
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
            ty: P(type_for_bitfield_width(ctx, width)),
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

    let stmts = Vec::with_capacity(bitfields.len() + 1);

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
        stmts: stmts,
        expr: Some(exprs),
        id: ast::DUMMY_NODE_ID,
        rules: ast::BlockCheckMode::Default,
        span: ctx.span
    };

    let node = ast::ImplItemKind::Method(
        ast::MethodSig {
            unsafety: ast::Unsafety::Normal,
            abi: Abi::Rust,
            decl: P(fndecl),
            generics: empty_generics(),
            explicit_self: respan(ctx.span, ast::SelfKind::Static),
            constness: ast::Constness::Const,
        }, P(block)
    );

    ast::ImplItem {
        id: ast::DUMMY_NODE_ID,
        ident: ctx.ext_cx.ident_of(&format!("new{}", bindgen_name)),
        vis: ast::Visibility::Public,
        attrs: vec!(),
        node: node,
        span: ctx.span,
    }
}

fn mk_blob_field(ctx: &GenCtx, name: &str, layout: Layout) -> Spanned<ast::StructField_> {
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
    respan(ctx.span, ast::StructField_ {
        kind: ast::NamedField(
            ctx.ext_cx.ident_of(name),
            ast::Visibility::Public,
        ),
        id: ast::DUMMY_NODE_ID,
        ty: data_ty,
        attrs: Vec::new()
    })
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

fn mk_repr_attr(ctx: &mut GenCtx, layout: Layout) -> ast::Attribute {
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

fn mk_deriving_copy_attr(ctx: &mut GenCtx) -> ast::Attribute {
    mk_deriving_attr(ctx, &["Copy", "Clone"])
}

fn mk_deriving_copy_and_maybe_debug_attr(ctx: &mut GenCtx) -> ast::Attribute {
    if ctx.options.derive_debug {
        mk_deriving_attr(ctx, &["Copy", "Clone", "Debug"])
    } else {
        mk_deriving_copy_attr(ctx)
    }
}

fn mk_deriving_attr(ctx: &mut GenCtx, attrs: &[&'static str]) -> ast::Attribute {
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

fn mk_doc_attr(ctx: &mut GenCtx, doc: &str) -> Vec<ast::Attribute> {
    if doc.is_empty() {
        return vec!();
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

    let mut attrs = Vec::new();
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
        empty_generics()
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
                let mut path = ctx.full_path_for_module(c.module_id);
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

fn cty_has_destructor(ty: &Type) -> bool {
    match ty {
        &TArray(ref t, _, _) => {
            cty_has_destructor(&**t)
        }
        &TComp(ref ci) => {
            let c = ci.borrow();
            if c.has_destructor || c.members.iter().any(|f| match f {
                &CompMember::Field(ref f) |
                &CompMember::CompField(_, ref f) =>
                    cty_has_destructor(&f.ty),
                _ => false,
            }) {
                return true;
            }
            c.ref_template.is_some()
        },
        &TNamed(ref ti) => {
            cty_has_destructor(&ti.borrow().ty)
        },
        _ => false,
    }
}

fn mk_ty(ctx: &GenCtx, global: bool, segments: &[String]) -> ast::Ty {
    mk_ty_args(ctx, global, segments, vec!())
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
                        lifetimes: vec!(),
                        types: OwnedSlice::from_vec(if i == segment_count - 1 { args.clone() } else { vec![] }),
                        bindings: OwnedSlice::empty(),
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
            attrs: None,
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
        lifetimes: Vec::new(),
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
        lifetimes: Vec::new(),
        decl: P(decl.clone())
    }));

    let segs = vec![
        ast::PathSegment {
            identifier: ctx.ext_cx.ident_of("std"),
            parameters: ast::PathParameters::AngleBracketed(ast::AngleBracketedParameterData {
                lifetimes: Vec::new(),
                types: OwnedSlice::empty(),
                bindings: OwnedSlice::empty(),
            }),
        },
        ast::PathSegment {
            identifier: ctx.ext_cx.ident_of("option"),
            parameters: ast::PathParameters::AngleBracketed(ast::AngleBracketedParameterData {
                lifetimes: Vec::new(),
                types: OwnedSlice::empty(),
                bindings: OwnedSlice::empty(),
            }),
        },
        ast::PathSegment {
            identifier: ctx.ext_cx.ident_of("Option"),
            parameters: ast::PathParameters::AngleBracketed(ast::AngleBracketedParameterData {
                lifetimes: Vec::new(),
                types: OwnedSlice::from_vec(vec!(
                    P(ast::Ty {
                        id: ast::DUMMY_NODE_ID,
                        node: fnty,
                        span: ctx.span
                    })
                )),
                bindings: OwnedSlice::empty(),
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
    let struct_name = ctx.ext_cx.ident_of(name);
    let fn_name = ctx.ext_cx.ident_of(&format!("bindgen_test_layout_{}", name));
    let item = quote_item!(&ctx.ext_cx,
        #[test]
        fn $fn_name() {
            assert_eq!(::std::mem::size_of::<$struct_name>(), $size);
        }).unwrap();
    item
}
