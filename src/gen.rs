use std::cell::RefCell;
use std::vec::Vec;
use std::rc::Rc;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use syntax::abi;
use syntax::ast;
use syntax::codemap::{Span, Spanned, respan, ExpnInfo, NameAndSpan, MacroBang};
use syntax::ext::base;
use syntax::ext::build::AstBuilder;
use syntax::ext::expand::ExpansionConfig;
use syntax::feature_gate::Features;
use syntax::owned_slice::OwnedSlice;
use syntax::parse;
use syntax::attr::mk_attr_id;
use syntax::ptr::P;

use super::LinkType;
use types::*;

struct GenCtx<'r> {
    ext_cx: base::ExtCtxt<'r>,
    unnamed_ty: usize,
    span: Span
}

fn first<A, B>((val, _): (A, B)) -> A {
    return val;
}

fn ref_eq<'a, 'b, T>(thing: &'a T, other: &'b T) -> bool {
    (thing as *const T) == (other as *const T)
}

fn to_intern_str(ctx: &mut GenCtx, s: String) -> parse::token::InternedString {
    let id = ctx.ext_cx.ident_of(&s[..]);
    parse::token::get_ident(id)
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

fn rust_id(ctx: &mut GenCtx, name: String) -> (String, bool) {
    let token = parse::token::Ident(ctx.ext_cx.ident_of(&name[..]), parse::token::Plain);
    if token.is_any_keyword() || "bool" == &name[..] {
        let mut s = "_".to_string();
        s.push_str(&name[..]);
        (s, true)
    } else {
        (name, false)
    }
}

fn rust_type_id(ctx: &mut GenCtx, name: String) -> String {
    if "bool" == &name[..] ||
        "uint" == &name[..] ||
        "u8" == &name[..] ||
        "u16" == &name[..] ||
        "u32" == &name[..] ||
        "f32" == &name[..] ||
        "f64" == &name[..] ||
        "i8" == &name[..] ||
        "i16" == &name[..] ||
        "i32" == &name[..] ||
        "i64" == &name[..] ||
        "Self" == &name[..] ||
        "str" == &name[..] {
        let mut s = "_".to_string();
        s.push_str(&name[..]);
        s
    } else {
        let (n, _) = rust_id(ctx, name);
        n
    }
}

fn unnamed_name(ctx: &mut GenCtx, name: String) -> String {
    return if name.is_empty() {
        ctx.unnamed_ty += 1;
        format!("Unnamed{}", ctx.unnamed_ty)
    } else {
        name
    };
}

fn comp_name(kind: CompKind, name: &String) -> String {
    match kind {
        CompKind::Struct => struct_name(name),
        CompKind::Union  => union_name(name),
    }
}

fn struct_name(name: &String) -> String {
    format!("Struct_{}", name)
}

fn union_name(name: &String) -> String {
    format!("Union_{}", name)
}

fn enum_name(name: &String) -> String {
    format!("Enum_{}", name)
}

pub fn gen_mod(links: &[(String, LinkType)], globs: Vec<Global>, span: Span) -> Vec<P<ast::Item>> {
    // Create a dummy ExtCtxt. We only need this for string interning and that uses TLS.
    let mut features = Features::new();
    features.allow_quote = true;
    let cfg = ExpansionConfig {
        crate_name: "xxx".to_string(),
        features: Some(&features),
        recursion_limit: 64,
        trace_mac: false,
    };
    let sess = &parse::new_parse_sess();
    let mut ctx = GenCtx {
        ext_cx: base::ExtCtxt::new(
            sess,
            Vec::new(),
            cfg,
        ),
        unnamed_ty: 0,
        span: span
    };
    ctx.ext_cx.bt_push(ExpnInfo {
        call_site: ctx.span,
        callee: NameAndSpan {
            name: String::new(),
            format: MacroBang,
            allow_internal_unstable: false,
            span: None
        }
    });
    let uniq_globs = tag_dup_decl(globs);

    let mut fs = vec!();
    let mut vs = vec!();
    let mut gs = vec!();
    for g in uniq_globs.into_iter() {
        match g {
            GOther => {}
            GFunc(_) => fs.push(g),
            GVar(_) => vs.push(g),
            _ => gs.push(g)
        }
    }

    let mut defs = vec!();
    gs = remove_redundant_decl(gs);

    for g in gs.into_iter() {
        match g {
            GType(ti) => {
                let t = ti.borrow().clone();
                defs.extend(ctypedef_to_rs(&mut ctx, t.name.clone(), &t.ty).into_iter())
            },
            GCompDecl(ci) => {
                {
                    let mut c = ci.borrow_mut();
                    c.name = unnamed_name(&mut ctx, c.name.clone());
                }
                let c = ci.borrow().clone();
                defs.push(opaque_to_rs(&mut ctx, comp_name(c.kind, &c.name)));
            },
            GComp(ci) => {
                {
                    let mut c = ci.borrow_mut();
                    c.name = unnamed_name(&mut ctx, c.name.clone());
                }
                let c = ci.borrow().clone();
                defs.extend(comp_to_rs(&mut ctx, c.kind, comp_name(c.kind, &c.name),
                                       c.layout, c.members).into_iter())
            },
            GEnumDecl(ei) => {
                {
                    let mut e = ei.borrow_mut();
                    e.name = unnamed_name(&mut ctx, e.name.clone());
                }
                let e = ei.borrow().clone();
                defs.push(opaque_to_rs(&mut ctx, enum_name(&e.name)));
            },
            GEnum(ei) => {
                {
                    let mut e = ei.borrow_mut();
                    e.name = unnamed_name(&mut ctx, e.name.clone());
                }
                let e = ei.borrow().clone();
                defs.extend(cenum_to_rs(&mut ctx, enum_name(&e.name), e.kind, e.items).into_iter())
            },
            _ => { }
        }
    }

    let vars = vs.into_iter().map(|v| {
        match v {
            GVar(vi) => {
                let v = vi.borrow();
                cvar_to_rs(&mut ctx, v.name.clone(), &v.ty, v.is_const)
            },
            _ => unreachable!()
        }
    }).collect();

    let funcs = {
        let func_list = fs.into_iter().map(|f| {
            match f {
                GFunc(vi) => {
                    let v = vi.borrow();
                    match v.ty {
                        TFuncPtr(ref sig) => {
                            let decl = cfunc_to_rs(&mut ctx, v.name.clone(),
                                                   &*sig.ret_ty, &sig.args[..],
                                                   sig.is_variadic);
                            (sig.abi, decl)
                        }
                        _ => unreachable!()
                    }
                },
                _ => unreachable!()
            }
        });

        let mut map: HashMap<abi::Abi, Vec<_>> = HashMap::new();
        for (abi, func) in func_list {
            match map.entry(abi) {
                Entry::Occupied(mut occ) => {
                    occ.get_mut().push(func);
                }
                Entry::Vacant(vac) => {
                    vac.insert(vec!(func));
                }
            }
        }
        map
    };

    if !Vec::is_empty(&vars) {
        defs.push(mk_extern(&mut ctx, links, vars, abi::C));
    }

    for (abi, funcs) in funcs.into_iter() {
        defs.push(mk_extern(&mut ctx, links, funcs, abi));
    }

    //let attrs = vec!(mk_attr_list(&mut ctx, "allow", ["dead_code", "non_camel_case_types", "uppercase_variables"]));

    defs
}

fn mk_extern(ctx: &mut GenCtx, links: &[(String, LinkType)],
             foreign_items: Vec<P<ast::ForeignItem>>,
             abi: abi::Abi) -> P<ast::Item> {
    let attrs = if links.is_empty() {
        Vec::new()
    } else {
        links.iter().map(|&(ref l, ref k)| {
            let k = match k {
                &LinkType::Default => None,
                &LinkType::Static => Some("static"),
                &LinkType::Framework => Some("framework")
            };
            let link_name = P(respan(ctx.span, ast::MetaNameValue(
                to_intern_str(ctx, "name".to_string()),
                respan(ctx.span, ast::LitStr(
                    to_intern_str(ctx, l.to_string()),
                    ast::CookedStr
                ))
            )));
            let link_args = match k {
                None => vec!(link_name),
                Some(ref k) => vec!(link_name, P(respan(ctx.span, ast::MetaNameValue(
                    to_intern_str(ctx, "kind".to_string()),
                    respan(ctx.span, ast::LitStr(
                        to_intern_str(ctx, k.to_string()),
                        ast::CookedStr
                    ))
                ))))
            };
            respan(ctx.span, ast::Attribute_ {
                id: mk_attr_id(),
                style: ast::AttrOuter,
                value: P(respan(ctx.span, ast::MetaList(
                    to_intern_str(ctx, "link".to_string()),
                    link_args)
                )),
                is_sugared_doc: false
            })
        }).collect()
    };

    let mut items = Vec::new();
    items.extend(foreign_items.into_iter());
    let ext = ast::ItemForeignMod(ast::ForeignMod {
        abi: abi,
        items: items
    });

    return P(ast::Item {
              ident: ctx.ext_cx.ident_of(""),
              attrs: attrs,
              id: ast::DUMMY_NODE_ID,
              node: ext,
              vis: ast::Inherited,
              span: ctx.span
           });
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

    return gs.into_iter().filter(|g|
        !typedefs.iter().any(|t| check_decl(g, t))
    ).collect();
}

fn tag_dup_decl(gs: Vec<Global>) -> Vec<Global> {
    fn check(name1: &str, name2: &str) -> bool {
        !name1.is_empty() && name1 == name2
    }

    fn check_dup(g1: &Global, g2: &Global) -> bool {
        match (g1, g2) {
          (&GType(ref ti1), &GType(ref ti2)) => {
              let a = ti1.borrow();
              let b = ti2.borrow();
              check(&a.name[..], &b.name[..])
          },
          (&GComp(ref ci1), &GComp(ref ci2)) => {
              let a = ci1.borrow();
              let b = ci2.borrow();
              check(&a.name[..], &b.name[..])
          },
          (&GCompDecl(ref ci1), &GCompDecl(ref ci2)) => {
              let a = ci1.borrow();
              let b = ci2.borrow();
              check(&a.name[..], &b.name[..])
          },
          (&GEnum(ref ei1), &GEnum(ref ei2)) => {
              let a = ei1.borrow();
              let b = ei2.borrow();
              check(&a.name[..], &b.name[..])
          },
          (&GEnumDecl(ref ei1), &GEnumDecl(ref ei2)) => {
              let a = ei1.borrow();
              let b = ei2.borrow();
              check(&a.name[..], &b.name[..])
          },
          (&GVar(ref vi1), &GVar(ref vi2)) => {
              let a = vi1.borrow();
              let b = vi2.borrow();
              check(&a.name[..], &b.name[..])
          },
          (&GFunc(ref vi1), &GFunc(ref vi2)) => {
              let a = vi1.borrow();
              let b = vi2.borrow();
              check(&a.name[..], &b.name[..])
          },
          _ => false
        }
    }

    if gs.is_empty() {
        return gs;
    }

    let len = gs.len();
    let mut res: Vec<Global> = vec!();
    res.push(gs[0].clone());

    for i in 1..len {
        let mut dup = false;
        for j in 0..i-1 {
            if check_dup(&gs[i], &gs[j]) {
                dup = true;
                break;
            }
        }
        if !dup {
            res.push(gs[i].clone());
        }
    }

    return res;
}

fn ctypedef_to_rs(ctx: &mut GenCtx, name: String, ty: &Type) -> Vec<P<ast::Item>> {
    fn mk_item(ctx: &mut GenCtx, name: String, ty: &Type) -> P<ast::Item> {
        let rust_name = rust_type_id(ctx, name);
        let rust_ty = cty_to_rs(ctx, ty);
        let base = ast::ItemTy(
            P(ast::Ty {
                id: ast::DUMMY_NODE_ID,
                node: rust_ty.node,
                span: ctx.span,
            }),
            empty_generics()
        );

        return P(ast::Item {
                  ident: ctx.ext_cx.ident_of(&rust_name[..]),
                  attrs: Vec::new(),
                  id: ast::DUMMY_NODE_ID,
                  node: base,
                  vis: ast::Public,
                  span: ctx.span
               });
    }

    return match *ty {
        TComp(ref ci) => {
            let is_empty = ci.borrow().name.is_empty();
            if is_empty {
                ci.borrow_mut().name = name.clone();
                let c = ci.borrow().clone();
                comp_to_rs(ctx, c.kind, name, c.layout, c.members)
            } else {
                vec!(mk_item(ctx, name, ty))
            }
        },
        TEnum(ref ei) => {
            let is_empty = ei.borrow().name.is_empty();
            if is_empty {
                ei.borrow_mut().name = name.clone();
                let e = ei.borrow().clone();
                cenum_to_rs(ctx, name, e.kind, e.items)
            } else {
                vec!(mk_item(ctx, name, ty))
            }
        },
        _ => vec!(mk_item(ctx, name, ty))
    }
}

fn comp_to_rs(ctx: &mut GenCtx, kind: CompKind, name: String,
              layout: Layout, members: Vec<CompMember>) -> Vec<P<ast::Item>> {
    match kind {
        CompKind::Struct => cstruct_to_rs(ctx, name, members),
        CompKind::Union =>  cunion_to_rs(ctx, name, layout, members),
    }
}

fn cstruct_to_rs(ctx: &mut GenCtx, name: String, members: Vec<CompMember>) -> Vec<P<ast::Item>> {
    let mut fields = vec!();
    let mut methods = vec!();
    // Nested composites may need to emit declarations and implementations as
    // they are encountered.  The declarations end up in 'extra' and are emitted
    // after the current struct.
    let mut extra = vec!();
    let mut unnamed: u32 = 0;
    let mut bitfields: u32 = 0;

    for m in members.iter() {
        let (opt_rc_c, opt_f) = match m {
            &CompMember::Field(ref f) => { (None, Some(f)) }
            &CompMember::Comp(ref rc_c) => { (Some(rc_c), None) }
            &CompMember::CompField(ref rc_c, ref f) => { (Some(rc_c), Some(f)) }
        };

        if let Some(f) = opt_f {
            let f_name = match f.bitfields {
                Some(_) => {
                    bitfields += 1;
                    format!("_bindgen_bitfield_{}_", bitfields)
                }
                None => rust_type_id(ctx, f.name.clone())
            };

            let f_ty = P(cty_to_rs(ctx, &f.ty));

            fields.push(respan(ctx.span, ast::StructField_ {
                kind: ast::NamedField(
                    ctx.ext_cx.ident_of(&f_name[..]),
                    ast::Public,
                ),
                id: ast::DUMMY_NODE_ID,
                ty: f_ty,
                attrs: Vec::new()
            }));
        }

        if let Some(rc_c) = opt_rc_c {
            let c = rc_c.borrow();
            if c.name.is_empty() {
                unnamed += 1;
                let field_name = format!("_bindgen_data_{}_", unnamed);
                fields.push(mk_blob_field(ctx, &field_name[..], c.layout));
                methods.extend(gen_comp_methods(ctx, &field_name[..], 0, c.kind, &c.members, &mut extra).into_iter());
            } else {
                extra.extend(comp_to_rs(ctx, c.kind, comp_name(c.kind, &c.name),
                                        c.layout, c.members.clone()).into_iter());
            }
        }
    }

    let ctor_id = if fields.is_empty() { Some(ast::DUMMY_NODE_ID) } else { None };
    let def = ast::ItemStruct(
        P(ast::StructDef {
           fields: fields,
           ctor_id: ctor_id,
        }),
        empty_generics()
    );

    let id = rust_type_id(ctx, name.clone());
    let struct_def = P(ast::Item { ident: ctx.ext_cx.ident_of(&id[..]),
        attrs: vec!(mk_repr_attr(ctx), mk_deriving_copy_attr(ctx)),
        id: ast::DUMMY_NODE_ID,
        node: def,
        vis: ast::Public,
        span: ctx.span
    });

    let mut items = vec!(struct_def);
    if !methods.is_empty() {
        let impl_ = ast::ItemImpl(
            ast::Unsafety::Normal,
            ast::ImplPolarity::Positive,
            empty_generics(),
            None,
            P(mk_ty(ctx, false, vec!(id))),
            methods
        );
        items.push(
            P(ast::Item {
                ident: ctx.ext_cx.ident_of(&name[..]),
                attrs: vec!(),
                id: ast::DUMMY_NODE_ID,
                node: impl_,
                vis: ast::Inherited,
                span: ctx.span}));
    }

    items.push(mk_clone_impl(ctx, &name[..]));
    items.push(mk_default_impl(ctx, &name[..]));
    items.extend(extra.into_iter());
    items
}

fn opaque_to_rs(ctx: &mut GenCtx, name: String) -> P<ast::Item> {
    let def = ast::ItemEnum(
        ast::EnumDef {
           variants: vec!()
        },
        empty_generics()
    );

    let id = rust_type_id(ctx, name);
    return P(ast::Item { ident: ctx.ext_cx.ident_of(&id[..]),
              attrs: Vec::new(),
              id: ast::DUMMY_NODE_ID,
              node: def,
              vis: ast::Public,
              span: ctx.span
           });
}

fn cunion_to_rs(ctx: &mut GenCtx, name: String, layout: Layout, members: Vec<CompMember>) -> Vec<P<ast::Item>> {
    fn mk_item(ctx: &mut GenCtx, name: String, item: ast::Item_, vis:
               ast::Visibility, attrs: Vec<ast::Attribute>) -> P<ast::Item> {
        return P(ast::Item {
            ident: ctx.ext_cx.ident_of(&name[..]),
            attrs: attrs,
            id: ast::DUMMY_NODE_ID,
            node: item,
            vis: vis,
            span: ctx.span
        });
    }

    let ci = Rc::new(RefCell::new(CompInfo::new(name.clone(), CompKind::Union, members.clone(), layout)));
    let union = TNamed(Rc::new(RefCell::new(TypeInfo::new(name.clone(), TComp(ci)))));

    // Nested composites may need to emit declarations and implementations as
    // they are encountered.  The declarations end up in 'extra' and are emitted
    // after the current union.
    let mut extra = vec!();

    let data_field_name = "_bindgen_data_";
    let data_field = mk_blob_field(ctx, data_field_name, layout);

    let def = ast::ItemStruct(
        P(ast::StructDef {
           fields: vec!(data_field),
           ctor_id: None,
        }),
        empty_generics()
    );
    let union_id = rust_type_id(ctx, name.clone());
    let union_attrs = vec!(mk_repr_attr(ctx), mk_deriving_copy_attr(ctx));
    let union_def = mk_item(ctx, union_id, def, ast::Public, union_attrs);

    let union_impl = ast::ItemImpl(
        ast::Unsafety::Normal,
        ast::ImplPolarity::Positive,
        empty_generics(),
        None,
        P(cty_to_rs(ctx, &union)),
        gen_comp_methods(ctx, data_field_name, 0, CompKind::Union, &members, &mut extra),
    );

    let mut items = vec!(
        union_def,
        mk_item(ctx, "".to_string(), union_impl, ast::Inherited, Vec::new())
    );

    items.push(mk_clone_impl(ctx, &name[..]));
    items.push(mk_default_impl(ctx, &name[..]));
    items.extend(extra.into_iter());
    items
}

fn cenum_to_rs(ctx: &mut GenCtx, name: String, kind: IKind, items: Vec<EnumItem>) -> Vec<P<ast::Item>> {
    let ty = TInt(kind, Layout::zero());
    let ty_id = rust_type_id(ctx, name);
    let ty_def = ctypedef_to_rs(ctx, ty_id, &ty);
    let val_ty = cty_to_rs(ctx, &ty);
    let mut def = ty_def;

    for it in items.iter() {
        let int_lit = ast::LitInt(
            it.val.abs() as u64,
            ast::UnsuffixedIntLit(if it.val < 0 { ast::Minus } else { ast::Plus })
        );

        let cst = ast::ItemConst(
            P(val_ty.clone()),
            ctx.ext_cx.expr_lit(ctx.span, int_lit)
        );

        let id = first(rust_id(ctx, it.name.clone()));
        let val_def = P(ast::Item {
                         ident: ctx.ext_cx.ident_of(&id[..]),
                         attrs: Vec::new(),
                         id: ast::DUMMY_NODE_ID,
                         node: cst,
                         vis: ast::Public,
                         span: ctx.span
                      });

        def.push(val_def);
    }

    return def;
}

/// Generates accessors for fields in nested structs and unions which must be
/// represented in Rust as an untyped array.  This process may generate
/// declarations and implementations that must be placed at the root level.
/// These are emitted into `extra`.
fn gen_comp_methods(ctx: &mut GenCtx, data_field: &str, data_offset: usize,
                    kind: CompKind, members: &Vec<CompMember>,
                    extra: &mut Vec<P<ast::Item>>) -> Vec<P<ast::ImplItem>> {
    let data_ident = ctx.ext_cx.ident_of(data_field);

    let mk_field_method = |ctx: &mut GenCtx, f: &FieldInfo, offset: usize| {
        // TODO: Implement bitfield accessors
        if f.bitfields.is_some() { return None; }

        let (f_name, _) = rust_id(ctx, f.name.clone());
        let f_name_ident = ctx.ext_cx.ident_of(&f_name[..]);
        let ret_ty = P(cty_to_rs(ctx, &TPtr(Box::new(f.ty.clone()), false, Layout::zero())));

        // When the offset is zero, generate slightly prettier code.
        let method = if offset == 0 {
            quote_item!(&ctx.ext_cx,
                impl X {
                    pub unsafe fn $f_name_ident(&mut self) -> $ret_ty {
                        ::std::mem::transmute(&self.$data_ident)
                    }
                }
            )
        } else {
            let offset_expr = &ctx.ext_cx.expr_int(ctx.span, offset as isize);
            quote_item!(&ctx.ext_cx,
                impl X {
                    pub unsafe fn $f_name_ident(&mut self) -> $ret_ty {
                        let raw: *mut u8 = ::std::mem::transmute(&self.$data_ident);
                        ::std::mem::transmute(raw.offset($offset_expr))
                    }
                }
            )
        };

        method.unwrap().and_then(|i| {
            match i.node {
                ast::ItemImpl(_, _, _, _, _, mut items) => {
                    items.pop()
                }
                _ => unreachable!("impl parsed to something other than impl")
            }
        })
    };

    let mut offset = data_offset;
    let mut methods = vec!();
    for m in members.iter() {
        let advance_by = match m {
            &CompMember::Field(ref f) => {
                methods.extend(mk_field_method(ctx, f, offset).into_iter());
                f.ty.size()
            }
            &CompMember::Comp(ref rc_c) => {
                let ref c = rc_c.borrow();
                methods.extend(gen_comp_methods(ctx, data_field, offset, c.kind,
                                                &c.members, extra).into_iter());
                c.layout.size
            }
            &CompMember::CompField(ref rc_c, ref f) => {
                methods.extend(mk_field_method(ctx, f, offset).into_iter());

                let c = rc_c.borrow();
                extra.extend(comp_to_rs(ctx, c.kind, comp_name(c.kind, &c.name),
                                        c.layout, c.members.clone()).into_iter());
                f.ty.size()
            }
        };
        match kind {
            CompKind::Struct => { offset += advance_by; }
            CompKind::Union  => { }
        }
    }
    methods
}

// Implements std::default::Default using std::mem::zeroed.
fn mk_default_impl(ctx: &GenCtx, ty_name: &str) -> P<ast::Item> {
    let name_ident = ctx.ext_cx.ident_of(ty_name);
    quote_item!(&ctx.ext_cx,
        impl ::std::default::Default for $name_ident {
            fn default() -> $name_ident { unsafe { ::std::mem::zeroed() } }
        }
    ).unwrap()
}

// Implements std::clone::Clone using dereferencing
fn mk_clone_impl(ctx: &GenCtx, ty_name: &str) -> P<ast::Item> {
    let name_ident = ctx.ext_cx.ident_of(ty_name);
    quote_item!(&ctx.ext_cx,
        impl ::std::clone::Clone for $name_ident {
            fn clone(&self) -> $name_ident { *self }
        }
    ).unwrap()
}

fn mk_blob_field(ctx: &GenCtx, name: &str, layout: Layout) -> Spanned<ast::StructField_> {
    let ty_name = match layout.align {
        1 => "u8",
        2 => "u16",
        4 => "u32",
        8 => "u64",
        _ => "u8",
    };
    let data_len = if ty_name == "u8" { layout.size } else { layout.size / layout.align };
    let base_ty = mk_ty(ctx, false, vec!(ty_name.to_string()));
    let data_ty = P(mk_arrty(ctx, &base_ty, data_len));
    respan(ctx.span, ast::StructField_ {
        kind: ast::NamedField(
            ctx.ext_cx.ident_of(name),
            ast::Public,
        ),
        id: ast::DUMMY_NODE_ID,
        ty: data_ty,
        attrs: Vec::new()
    })
}

fn mk_link_name_attr(ctx: &mut GenCtx, name: String) -> ast::Attribute {
    let lit = respan(ctx.span, ast::LitStr(
        to_intern_str(ctx, name),
        ast::CookedStr
    ));
    let attr_val = P(respan(ctx.span, ast::MetaNameValue(
        to_intern_str(ctx, "link_name".to_string()), lit
    )));
    let attr = ast::Attribute_ {
        id: mk_attr_id(),
        style: ast::AttrOuter,
        value: attr_val,
        is_sugared_doc: false
    };
    respan(ctx.span, attr)
}

fn mk_repr_attr(ctx: &mut GenCtx) -> ast::Attribute {
    let attr_val = P(respan(ctx.span, ast::MetaList(
        to_intern_str(ctx, "repr".to_string()),
        vec!(P(respan(ctx.span, ast::MetaWord(to_intern_str(ctx, "C".to_string())))))
    )));

    respan(ctx.span, ast::Attribute_ {
        id: mk_attr_id(),
        style: ast::AttrOuter,
        value: attr_val,
        is_sugared_doc: false
    })
}

fn mk_deriving_copy_attr(ctx: &mut GenCtx) -> ast::Attribute {
    let attr_val = P(respan(ctx.span, ast::MetaList(
        to_intern_str(ctx, "derive".to_string()),
        vec!(P(respan(ctx.span, ast::MetaWord(to_intern_str(ctx, "Copy".to_string())))))
    )));

    respan(ctx.span, ast::Attribute_ {
        id: mk_attr_id(),
        style: ast::AttrOuter,
        value: attr_val,
        is_sugared_doc: false
    })
}

fn cvar_to_rs(ctx: &mut GenCtx, name: String,
                                ty: &Type,
                                is_const: bool) -> P<ast::ForeignItem> {
    let (rust_name, was_mangled) = rust_id(ctx, name.clone());

    let mut attrs = Vec::new();
    if was_mangled {
        attrs.push(mk_link_name_attr(ctx, name));
    }

    return P(ast::ForeignItem {
              ident: ctx.ext_cx.ident_of(&rust_name[..]),
              attrs: attrs,
              node: ast::ForeignItemStatic(P(cty_to_rs(ctx, ty)), !is_const),
              id: ast::DUMMY_NODE_ID,
              span: ctx.span,
              vis: ast::Public,
           });
}

fn cfuncty_to_rs(ctx: &mut GenCtx,
                 rty: &Type,
                 aty: &[(String, Type)],
                 var: bool) -> ast::FnDecl {

    let ret = P(match *rty {
        TVoid => ast::Ty {
            id: ast::DUMMY_NODE_ID,
            node: ast::TyTup(vec![]),
            span: ctx.span
        },
        _ => cty_to_rs(ctx, rty)
    });

    let mut unnamed: usize = 0;
    let args: Vec<ast::Arg> = aty.iter().map(|arg| {
        let (ref n, ref t) = *arg;

        let arg_name = if n.is_empty() {
            unnamed += 1;
            format!("arg{}", unnamed)
        } else {
            first(rust_id(ctx, n.clone()))
        };

        // From the C90 standard (http://c0x.coding-guidelines.com/6.7.5.3.html)
        // 1598 - A declaration of a parameter as “array of type” shall be
        // adjusted to “qualified pointer to type”, where the type qualifiers
        // (if any) are those specified within the [ and ] of the array type
        // derivation.
        let arg_ty = P(match t {
            &TArray(ref typ, _, ref l) => cty_to_rs(ctx, &TPtr(typ.clone(), false, l.clone())),
            _ => cty_to_rs(ctx, t),
        });

        ast::Arg {
            ty: arg_ty,
            pat: P(ast::Pat {
                 id: ast::DUMMY_NODE_ID,
                 node: ast::PatIdent(
                     ast::BindByValue(ast::MutImmutable),
                     respan(ctx.span, ctx.ext_cx.ident_of(&arg_name[..])),
                     None
                 ),
                 span: ctx.span
            }),
            id: ast::DUMMY_NODE_ID,
        }
    }).collect();

    let var = !args.is_empty() && var;
    return ast::FnDecl {
        inputs: args,
        output: ast::Return(ret),
        variadic: var
    };
}

fn cfunc_to_rs(ctx: &mut GenCtx, name: String, rty: &Type,
               aty: &[(String, Type)],
               var: bool) -> P<ast::ForeignItem> {
    let var = !aty.is_empty() && var;
    let decl = ast::ForeignItemFn(
        P(cfuncty_to_rs(ctx, rty, aty, var)),
        empty_generics()
    );

    let (rust_name, was_mangled) = rust_id(ctx, name.clone());

    let mut attrs = Vec::new();
    if was_mangled {
        attrs.push(mk_link_name_attr(ctx, name));
    }

    return P(ast::ForeignItem {
              ident: ctx.ext_cx.ident_of(&rust_name[..]),
              attrs: attrs,
              node: decl,
              id: ast::DUMMY_NODE_ID,
              span: ctx.span,
              vis: ast::Public,
           });
}

fn cty_to_rs(ctx: &mut GenCtx, ty: &Type) -> ast::Ty {
    return match ty {
        &TVoid => mk_ty(ctx, true, vec!("libc".to_string(), "c_void".to_string())),
        &TInt(i, ref layout) => match i {
            IBool => {
                let ty_name = match layout.size {
                    1 => "u8",
                    2 => "u16",
                    4 => "u32",
                    8 => "u64",
                    _ => "u8",
                };
                mk_ty(ctx, false, vec!(ty_name.to_string()))
            },
            ISChar => mk_ty(ctx, true, vec!("libc".to_string(), "c_char".to_string())),
            IUChar => mk_ty(ctx, true, vec!("libc".to_string(), "c_uchar".to_string())),
            IInt => mk_ty(ctx, true, vec!("libc".to_string(), "c_int".to_string())),
            IUInt => mk_ty(ctx, true, vec!("libc".to_string(), "c_uint".to_string())),
            IShort => mk_ty(ctx, true, vec!("libc".to_string(), "c_short".to_string())),
            IUShort => mk_ty(ctx, true, vec!("libc".to_string(), "c_ushort".to_string())),
            ILong => mk_ty(ctx, true, vec!("libc".to_string(), "c_long".to_string())),
            IULong => mk_ty(ctx, true, vec!("libc".to_string(), "c_ulong".to_string())),
            ILongLong => mk_ty(ctx, true, vec!("libc".to_string(), "c_longlong".to_string())),
            IULongLong => mk_ty(ctx, true, vec!("libc".to_string(), "c_ulonglong".to_string()))
        },
        &TFloat(f, _) => match f {
            FFloat => mk_ty(ctx, true, vec!("libc".to_string(), "c_float".to_string())),
            FDouble => mk_ty(ctx, true, vec!("libc".to_string(), "c_double".to_string()))
        },
        &TPtr(ref t, is_const, _) => {
            let id = cty_to_rs(ctx, &**t);
            mk_ptrty(ctx, &id, is_const)
        },
        &TArray(ref t, s, _) => {
            let ty = cty_to_rs(ctx, &**t);
            mk_arrty(ctx, &ty, s)
        },
        &TFuncPtr(ref sig) => {
            let decl = cfuncty_to_rs(ctx, &*sig.ret_ty, &sig.args[..], sig.is_variadic);
            mk_fnty(ctx, &decl, sig.abi)
        },
        &TFuncProto(ref sig) => {
            let decl = cfuncty_to_rs(ctx, &*sig.ret_ty, &sig.args[..], sig.is_variadic);
            mk_fn_proto_ty(ctx, &decl, sig.abi)
        },
        &TNamed(ref ti) => {
            let id = rust_type_id(ctx, ti.borrow().name.clone());
            mk_ty(ctx, false, vec!(id))
        },
        &TComp(ref ci) => {
            let mut c = ci.borrow_mut();
            c.name = unnamed_name(ctx, c.name.clone());
            mk_ty(ctx, false, vec!(comp_name(c.kind, &c.name)))
        },
        &TEnum(ref ei) => {
            let mut e = ei.borrow_mut();
            e.name = unnamed_name(ctx, e.name.clone());
            mk_ty(ctx, false, vec!(enum_name(&e.name)))
        }
    };
}

fn mk_ty(ctx: &GenCtx, global: bool, segments: Vec<String>) -> ast::Ty {
    let ty = ast::TyPath(
        None,
        ast::Path {
            span: ctx.span,
            global: global,
            segments: segments.iter().map(|s| {
                ast::PathSegment {
                    identifier: ctx.ext_cx.ident_of(&s[..]),
                    parameters: ast::AngleBracketedParameters(ast::AngleBracketedParameterData {
                        lifetimes: Vec::new(),
                        types: OwnedSlice::empty(),
                        bindings: OwnedSlice::empty(),
                    }),
                }
            }).collect()
        },
    );

    return ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: ty,
        span: ctx.span
    };
}

fn mk_ptrty(ctx: &mut GenCtx, base: &ast::Ty, is_const: bool) -> ast::Ty {
    let ty = ast::TyPtr(ast::MutTy {
        ty: P(base.clone()),
        mutbl: if is_const { ast::MutImmutable } else { ast::MutMutable }
    });

    return ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: ty,
        span: ctx.span
    };
}

fn mk_arrty(ctx: &GenCtx, base: &ast::Ty, n: usize) -> ast::Ty {
    let int_lit = ast::LitInt(n as u64, ast::UnsignedIntLit(ast::TyUs));
    let sz = ast::ExprLit(P(respan(ctx.span, int_lit)));
    let ty = ast::TyFixedLengthVec(
        P(base.clone()),
        P(ast::Expr {
            id: ast::DUMMY_NODE_ID,
            node: sz,
            span: ctx.span
        })
    );

    return ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: ty,
        span: ctx.span
    };
}

fn mk_fn_proto_ty(ctx: &mut GenCtx, decl: &ast::FnDecl, abi: abi::Abi) -> ast::Ty {
    let fnty = ast::TyBareFn(P(ast::BareFnTy {
        unsafety: ast::Unsafety::Normal,
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

fn mk_fnty(ctx: &mut GenCtx, decl: &ast::FnDecl, abi: abi::Abi) -> ast::Ty {
    let fnty = ast::TyBareFn(P(ast::BareFnTy {
        unsafety: ast::Unsafety::Normal,
        abi: abi,
        lifetimes: Vec::new(),
        decl: P(decl.clone())
    }));

    let segs = vec![
        ast::PathSegment {
            identifier: ctx.ext_cx.ident_of("std"),
            parameters: ast::AngleBracketedParameters(ast::AngleBracketedParameterData {
                lifetimes: Vec::new(),
                types: OwnedSlice::empty(),
                bindings: OwnedSlice::empty(),
            }),
        },
        ast::PathSegment {
            identifier: ctx.ext_cx.ident_of("option"),
            parameters: ast::AngleBracketedParameters(ast::AngleBracketedParameterData {
                lifetimes: Vec::new(),
                types: OwnedSlice::empty(),
                bindings: OwnedSlice::empty(),
            }),
        },
        ast::PathSegment {
            identifier: ctx.ext_cx.ident_of("Option"),
            parameters: ast::AngleBracketedParameters(ast::AngleBracketedParameterData {
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

    return ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: ast::TyPath(
            None,
            ast::Path {
                span: ctx.span,
                global: true,
                segments: segs
            },
        ),
        span: ctx.span
    };
}
