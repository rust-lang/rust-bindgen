use std;
use std::mem;
use std::cell::RefCell;
use std::vec::Vec;
use std::rc::Rc;
use std::collections::HashMap;
use std::ascii::AsciiExt;

use syntax::abi;
use syntax::ast;
use syntax::codemap::{ExpnInfo, MacroBang, NameAndSpan, Span, respan};
use syntax::ext::base;
use syntax::ext::build::AstBuilder;
use syntax::ext::expand::ExpansionConfig;
use syntax::ext::quote::rt::ToTokens;
use syntax::feature_gate::Features;
use syntax::parse;
use syntax::parse::token::InternedString;
use syntax::attr::mk_attr_id;
use syntax::ptr::P;
use syntax::print::pprust::tts_to_string;

use super::{BindgenOptions, LinkType};
use types::*;

struct GenCtx<'r> {
    ext_cx: base::ExtCtxt<'r>,
    unnamed_ty: usize,
    span: Span,
}

fn ref_eq<T>(thing: &T, other: &T) -> bool {
    (thing as *const T) == (other as *const T)
}

fn is_type(name: &str) -> bool {
    ["bool", "uint", "u8", "u16", "u32", "u64", "f32", "f64", "i8", "i16", "i32", "i64", "Self",
     "self", "usize", "isize"]
        .contains(&name)
}

fn rust_id(ctx: &mut GenCtx, mut name: &str, remove_prefix: &str) -> (String, bool) {
    let mut modified = false;
    if remove_prefix != "" && name.len() >= remove_prefix.len() &&
       name[..remove_prefix.len()].eq_ignore_ascii_case(remove_prefix) {
        name = &name[remove_prefix.len()..];
        modified = true;
    }
    let token = parse::token::Ident(ctx.ext_cx.ident_of(name));
    if token.is_any_keyword() || is_type(name) {
        (format!("{}_", name), true)
    } else {
        (name.into(), modified)
    }
}

const UNNAMED_PREFIX: &'static str = "Unnamed";

fn unnamed_name(ctx: &mut GenCtx, name: &str) -> String {
    if name.is_empty() {
        ctx.unnamed_ty += 1;
        format!("{}{}", UNNAMED_PREFIX, ctx.unnamed_ty)
    } else {
        name.into()
    }
}

fn comp_name(kind: CompKind, name: &str) -> String {
    match kind {
        CompKind::Struct => struct_name(name),
        CompKind::Union => union_name(name),
    }
}

fn struct_name(name: &str) -> String {
    if name.starts_with(UNNAMED_PREFIX) {
        format!("Struct_{}", name)
    } else {
        name.into()
    }
}

fn union_name(name: &str) -> String {
    if name.starts_with(UNNAMED_PREFIX) {
        format!("Union_{}", name)
    } else {
        name.into()
    }
}

fn enum_name(name: &str) -> String {
    if name.starts_with(UNNAMED_PREFIX) {
        format!("Enum_{}", name)
    } else {
        name.into()
    }
}

fn extract_definitions(ctx: &mut GenCtx,
                       options: &BindgenOptions,
                       globals: &[Global])
                       -> Vec<P<ast::Item>> {
    let mut defs = vec![];

    for g in globals {
        match *g {
            GType(ref ti) => {
                let t = ti.borrow();

                let is_cyclic = {
                    let n;
                    let c;
                    let e;
                    match t.ty {
                        TNamed(ref ni) => { n=ni.borrow(); Some(&n.name) },
                        TComp(ref ci) =>  { c=ci.borrow(); Some(&c.name) },
                        TEnum(ref ei) =>  { e=ei.borrow(); Some(&e.name) },
                        _ => None,
                    }.map(|alias|
                        rust_id(ctx, &t.name, &options.remove_prefix).0 == rust_id(ctx, alias, &options.remove_prefix).0
                    ).unwrap_or(false)
                    // important: need to end borrow of n, c, e, here
                };
                if !is_cyclic {
                    defs.extend(ctypedef_to_rs(ctx, options, options.derive_debug, &t.name, &t.ty))
                }
            }
            GCompDecl(ref ci) => {
                {
                    let mut c = ci.borrow_mut();
                    c.name = unnamed_name(ctx, &c.name);
                }
                let c = ci.borrow().clone();
                defs.push(opaque_to_rs(ctx, &comp_name(c.kind, &c.name), &options.remove_prefix));
            }
            GComp(ref ci) => {
                {
                    let mut c = ci.borrow_mut();
                    c.name = unnamed_name(ctx, &c.name);
                }
                let c = ci.borrow().clone();
                defs.extend(comp_to_rs(ctx,
                                       c.kind,
                                       comp_name(c.kind, &c.name),
                                       options,
                                       options.derive_debug,
                                       c.layout,
                                       c.members)
                                .into_iter())
            }
            GEnumDecl(ref ei) => {
                {
                    let mut e = ei.borrow_mut();
                    e.name = unnamed_name(ctx, &e.name);
                }
                let e = ei.borrow().clone();
                defs.push(opaque_to_rs(ctx, &enum_name(&e.name), &options.remove_prefix));
            }
            GEnum(ref ei) => {
                {
                    let mut e = ei.borrow_mut();
                    e.name = unnamed_name(ctx, &e.name);
                }
                let e = ei.borrow();
                defs.extend(cenum_to_rs(ctx,
                                        options,
                                        options.derive_debug,
                                        &enum_name(&e.name),
                                        e.kind,
                                        e.layout,
                                        &e.items));
            }
            GVar(ref vi) => {
                let v = vi.borrow();
                let ty = cty_to_rs(ctx, &v.ty, options);
                defs.push(const_to_rs(ctx, &v.name, v.val.unwrap(), ty, options));
            }
            _ => {}
        }
    }

    defs
}

fn extract_functions(ctx: &mut GenCtx,
                     fs: &[Global],
                     options: &BindgenOptions)
                     -> HashMap<abi::Abi, Vec<ast::ForeignItem>> {
    let func_list = fs.iter().map(|f| {
        match *f {
            GFunc(ref vi) => {
                let v = vi.borrow();
                match v.ty {
                    TFuncPtr(ref sig, _) => {
                        let decl = cfunc_to_rs(ctx,
                                               v.name.clone(),
                                               &*sig.ret_ty,
                                               &sig.args[..],
                                               sig.is_variadic,
                                               options);
                        (sig.abi, decl)
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    });

    let mut map = HashMap::new();
    for (abi, func) in func_list {
        map.entry(abi).or_insert_with(Vec::new).push(func);
    }
    map
}

/// Converts `typedef struct {...} Test` to rust `struct Test {...}`
fn remove_unnamed(globals: &mut Vec<Global>) {
    let mut i = 1;
    while i < globals.len() {
        let mut remove = false;
        if let GType(ref t) = globals[i] {
            let t = t.borrow();
            match t.ty {
                TComp(ref c) => {
                    let mut c = c.borrow_mut();
                    if c.name.is_empty() {
                        c.name = t.name.clone();
                        remove = true;
                    } else if c.name == t.name {
                        remove = true;
                    }
                }
                TEnum(ref e) => {
                    let mut e = e.borrow_mut();
                    if e.name.is_empty() {
                        e.name = t.name.clone();
                        remove = true;
                    } else if e.name == t.name {
                        remove = true;
                    }
                }
                _ => (),
            }
        }

        if remove {
            globals.remove(i);
        } else {
            i += 1;
        }
    }
}

pub fn gen_mod(options: &BindgenOptions,
               globs: Vec<Global>,
               span: Span)
               -> (Vec<P<ast::Item>>, Vec<ast::Attribute>) {
    // Create a dummy ExtCtxt. We only need this for string interning and that uses TLS.
    let mut features = Features::new();
    features.quote = true;
    let cfg = {
        let mut cfg = ExpansionConfig::default("xxx".into());
        cfg.features = Some(&features);
        cfg
    };
    let sess = &parse::ParseSess::new();
    let mut macro_loader = base::DummyMacroLoader;
    let mut ctx = GenCtx {
        ext_cx: base::ExtCtxt::new(sess,
                                   Vec::new(),
                                   cfg,
                                   &mut macro_loader),
        unnamed_ty: 0,
        span: span,
    };
    ctx.ext_cx.bt_push(ExpnInfo {
        call_site: ctx.span,
        callee: NameAndSpan {
            format: MacroBang(parse::token::intern("")),
            allow_internal_unstable: false,
            span: None,
        },
    });
    let uniq_globs = tag_dup_decl(&globs);

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
                        _ => unreachable!(),
                    }
                };
                if is_int_const {
                    gs.push(g);
                } else {
                    vs.push(g);
                }
            }
            _ => gs.push(g),
        }
    }

    gs = remove_redundant_decl(gs);
    remove_unnamed(&mut gs);
    let mut defs = extract_definitions(&mut ctx, options, &gs);

    let vars = vs.into_iter()
                 .map(|v| {
                     match v {
                         GVar(vi) => {
                             let v = vi.borrow();
                             cvar_to_rs(&mut ctx, v.name.clone(), &v.ty, v.is_const, options)
                         }
                         _ => unreachable!(),
                     }
                 })
                 .collect();

    let funcs = extract_functions(&mut ctx, &fs, options);

    if !Vec::is_empty(&vars) {
        defs.push(mk_extern(&mut ctx, &options.links, vars, abi::Abi::C));
    }

    for (abi, funcs) in funcs.into_iter() {
        defs.push(mk_extern(&mut ctx, &options.links, funcs, abi));
    }

    // let attrs = vec!(mk_attr_list(&mut ctx, "allow", ));
    let mod_attrs = vec![mk_attr_style(&mut ctx,
                                       "allow",
                                       &["dead_code",
                                         "non_camel_case_types",
                                         "non_upper_case_globals",
                                         "non_snake_case"],
                                       ast::AttrStyle::Inner)];
    (defs, mod_attrs)
}

fn mk_extern(ctx: &mut GenCtx,
             links: &[(String, LinkType)],
             foreign_items: Vec<ast::ForeignItem>,
             abi: abi::Abi)
             -> P<ast::Item> {
    let attrs = if links.is_empty() {
        Vec::new()
    } else {
        links.iter()
             .map(|&(ref l, ref k)| {
                 let k = match *k {
                     LinkType::Static => Some("static"),
                     LinkType::Dynamic => Some("dylib"),
                     LinkType::Framework => Some("framework"),
                 };
                 let link_name = P(respan(ctx.span, ast::MetaItemKind::NameValue(
                InternedString::new("name"),
                respan(ctx.span, ast::LitKind::Str(
                    ctx.ext_cx.name_of(l).as_str(),
                    ast::StrStyle::Cooked
                ))
            )));
                 let link_args = match k {
                     None => vec![link_name],
                     Some(ref k) => {
                         vec!(link_name, P(respan(ctx.span, ast::MetaItemKind::NameValue(
                    InternedString::new("kind"),
                    respan(ctx.span, ast::LitKind::Str(
                        ctx.ext_cx.name_of(k).as_str(),
                        ast::StrStyle::Cooked
                    ))
                ))))
                     }
                 };
                 respan(ctx.span,
                        ast::Attribute_ {
                            id: mk_attr_id(),
                            style: ast::AttrStyle::Outer,
                            value: P(respan(ctx.span,
                                            ast::MetaItemKind::List(InternedString::new("link"),
                                                                    link_args))),
                            is_sugared_doc: false,
                        })
             })
             .collect()
    };

    let mut items = Vec::new();
    items.extend(foreign_items.into_iter());
    let ext = ast::ItemKind::ForeignMod(ast::ForeignMod {
        abi: abi,
        items: items,
    });

    P(ast::Item {
        ident: ctx.ext_cx.ident_of(""),
        attrs: attrs,
        id: ast::DUMMY_NODE_ID,
        node: ext,
        vis: ast::Visibility::Inherited,
        span: ctx.span,
    })
}

fn remove_redundant_decl(gs: Vec<Global>) -> Vec<Global> {
    fn check_decl(a: &Global, ty: &Type) -> bool {
        match *a {
            GComp(ref ci1) => {
                match *ty {
                    TComp(ref ci2) => ref_eq(ci1, ci2) && ci1.borrow().name.is_empty(),
                    _ => false,
                }
            }
            GEnum(ref ei1) => {
                match *ty {
                    TEnum(ref ei2) => ref_eq(ei1, ei2) && ei1.borrow().name.is_empty(),
                    _ => false,
                }
            }
            _ => false,
        }
    }

    let typedefs: Vec<Type> = gs.iter()
                                .filter_map(|g| {
                                    match *g {
                                        GType(ref ti) => Some(ti.borrow().ty.clone()),
                                        _ => None,
                                    }
                                })
                                .collect();

    gs.into_iter()
      .filter(|g| !typedefs.iter().any(|t| check_decl(g, t)))
      .collect()
}

fn tag_dup_decl(gs: &[Global]) -> Vec<Global> {
    fn check(name1: &str, name2: &str) -> bool {
        !name1.is_empty() && name1 == name2
    }

    fn is_dup(g1: &Global, g2: &Global) -> bool {
        match (g1, g2) {
            (&GType(ref ti1), &GType(ref ti2)) => {
                let a = ti1.borrow();
                let b = ti2.borrow();
                check(&a.name[..], &b.name[..])
            }
            (&GComp(ref ci1), &GComp(ref ci2)) |
            (&GCompDecl(ref ci1), &GCompDecl(ref ci2)) => {
                let a = ci1.borrow();
                let b = ci2.borrow();
                check(&a.name[..], &b.name[..])
            }
            (&GEnum(ref ei1), &GEnum(ref ei2)) |
            (&GEnumDecl(ref ei1), &GEnumDecl(ref ei2)) => {
                let a = ei1.borrow();
                let b = ei2.borrow();
                check(&a.name[..], &b.name[..])
            }
            (&GVar(ref vi1), &GVar(ref vi2)) |
            (&GFunc(ref vi1), &GFunc(ref vi2)) => {
                let a = vi1.borrow();
                let b = vi2.borrow();
                check(&a.name[..], &b.name[..])
            }
            _ => false,
        }
    }

    if gs.is_empty() {
        return vec![];
    }

    let mut res: Vec<Global> = vec![];
    res.push(gs[0].clone());

    for (i, gsi) in gs.iter().enumerate().skip(1) {
        let dup = gs.iter().take(i).any(|item| is_dup(item, gsi));
        if !dup {
            res.push(gsi.clone());
        }
    }

    res
}

/// Converts a C typedef to Rust AST Items.
fn ctypedef_to_rs(ctx: &mut GenCtx,
                  options: &BindgenOptions,
                  derive_debug: bool,
                  name: &str,
                  ty: &Type)
                  -> Vec<P<ast::Item>> {
    let mk_item = |ctx: &mut GenCtx, name: &str, ty: &Type| -> P<ast::Item> {
        let rust_ty = match &name[..] {
            // Alias these types to `usize`, e.g. `type size_t = usize`, instead of aliasing them to
            // an arch-dependent type, i.e. `type size_t = u64` on 64-bit archs and
            // `type size_t = u32` on 32-bit archs
            "size_t" | "uintptr_t" => mk_ty(ctx, false, vec!["usize".to_string()]),
            // Same as above
            "ptrdiff_t" | "intptr_t" | "ssize_t" => mk_ty(ctx, false, vec!["isize".to_string()]),
            "uint8_t" => mk_ty(ctx, false, vec!["u8".to_string()]),
            "int8_t" => mk_ty(ctx, false, vec!["i8".to_string()]),
            "uint16_t" => mk_ty(ctx, false, vec!["u16".to_string()]),
            "int16_t" => mk_ty(ctx, false, vec!["i16".to_string()]),
            "uint32_t" => mk_ty(ctx, false, vec!["u32".to_string()]),
            "int32_t" => mk_ty(ctx, false, vec!["i32".to_string()]),
            "uint64_t" => mk_ty(ctx, false, vec!["u64".to_string()]),
            "int64_t" => mk_ty(ctx, false, vec!["i64".to_string()]),
            _ => cty_to_rs(ctx, ty, options),
        };
        let rust_name = rust_id(ctx, name, &options.remove_prefix).0;
        let base = ast::ItemKind::Ty(P(ast::Ty {
                                         id: ast::DUMMY_NODE_ID,
                                         node: rust_ty.node,
                                         span: ctx.span,
                                     }),
                                     ast::Generics::default());

        P(ast::Item {
            ident: ctx.ext_cx.ident_of(&rust_name),
            attrs: Vec::new(),
            id: ast::DUMMY_NODE_ID,
            node: base,
            vis: ast::Visibility::Public,
            span: ctx.span,
        })
    };

    match *ty {
        TComp(ref ci) => {
            let is_empty = ci.borrow().name.is_empty();
            if is_empty {
                ci.borrow_mut().name = name.into();
                let c = ci.borrow().clone();
                comp_to_rs(ctx,
                           c.kind,
                           name.into(),
                           options,
                           derive_debug,
                           c.layout,
                           c.members)
            } else {
                vec![mk_item(ctx, name, ty)]
            }
        }
        TEnum(ref ei) => {
            let is_empty = ei.borrow().name.is_empty();
            if is_empty {
                ei.borrow_mut().name = name.into();
                let e = ei.borrow();
                cenum_to_rs(ctx, options, derive_debug, name, e.kind, e.layout, &e.items)
            } else {
                vec![mk_item(ctx, name, ty)]
            }
        }
        _ => vec![mk_item(ctx, name, ty)],
    }
}

/// Converts a C composed type (struct or union) to Rust AST Items.
fn comp_to_rs(ctx: &mut GenCtx,
              kind: CompKind,
              name: String,
              options: &BindgenOptions,
              derive_debug: bool,
              layout: Layout,
              members: Vec<CompMember>)
              -> Vec<P<ast::Item>> {
    match kind {
        CompKind::Struct => cstruct_to_rs(ctx, &name, options, derive_debug, layout, members),
        CompKind::Union => cunion_to_rs(ctx, name, options, derive_debug, layout, members),
    }
}

fn gen_padding_fields(ctx: &mut GenCtx,
                      idx: usize,
                      offset: usize,
                      padding_size: usize)
                      -> Vec<ast::StructField> {
    const MAX_ARRAY_CLONE_LEN: usize = 32; // impl<T: Copy> Clone for [T; 32]

    let u64_size = mem::size_of::<u64>();
    let max_field_size = u64_size * MAX_ARRAY_CLONE_LEN;
    let u64_ty = P(mk_ty(ctx, false, vec!["u64".to_owned()]));
    let u8_ty = P(mk_ty(ctx, false, vec!["u8".to_owned()]));
    let mut size = padding_size;

    let u64_padding_size = u64_size - (offset % u64_size);

    if (size - u64_padding_size) > u64_size && u64_padding_size != u64_size {
        size -= u64_padding_size;
    }

    let mut fields = (0..(size / max_field_size))
                         .map(|_| (&u64_ty, MAX_ARRAY_CLONE_LEN))
                         .collect::<Vec<(&P<ast::Ty>, usize)>>();

    let u64_num = (size % max_field_size) / u64_size;

    if u64_num > 0 {
        fields.push((&u64_ty, u64_num));
    }

    let u8_num = size % u64_size;

    if u8_num > 0 {
        fields.push((&u8_ty, u8_num));
    }

    fields.iter()
          .enumerate()
          .map(|(i, &(ref el_ty, el_num))| {
              let name = format!("_bindgen_padding_{}_", idx + i);

              let padding_ty = P(mk_arrty(ctx, el_ty, el_num));

              ast::StructField {
                  span: ctx.span,
                  vis: ast::Visibility::Inherited,
                  ident: Some(ctx.ext_cx.ident_of(&name[..])),
                  id: ast::DUMMY_NODE_ID,
                  ty: padding_ty,
                  attrs: Vec::new(),
              }
          })
          .collect()
}

/// Converts a C struct to Rust AST Items.
fn cstruct_to_rs(ctx: &mut GenCtx,
                 name: &str,
                 options: &BindgenOptions,
                 derive_debug: bool,
                 layout: Layout,
                 members: Vec<CompMember>)
                 -> Vec<P<ast::Item>> {
    let mut fields: Vec<ast::StructField> = vec![];
    let mut methods = vec![];
    // Nested composites may need to emit declarations and implementations as
    // they are encountered.  The declarations end up in 'extra' and are emitted
    // after the current struct.
    let mut extra = vec![];
    let mut unnamed: u32 = 0;
    let mut bitfields: u32 = 0;
    let mut paddings = 0;
    let mut offset = 0;

    // Waiting for https://github.com/rust-lang/rfcs/issues/1038
    let mut can_derive_debug = derive_debug;
    let mut can_derive_clone = true;

    for m in &members {
        debug!("convert field {} {:?}", m.name(), m);

        let (opt_rc_c, opt_rc_e, opt_f) = match *m {
            CompMember::Field(ref f) => (None, None, Some(f)),
            CompMember::Comp(ref rc_c) => (Some(rc_c), None, None),
            CompMember::CompField(ref rc_c, ref f) => (Some(rc_c), None, Some(f)),
            CompMember::Enum(ref rc_e) => (None, Some(rc_e), None),
            CompMember::EnumField(ref rc_e, ref f) => (None, Some(rc_e), Some(f)),
        };

        if !layout.packed && m.layout().align != 0 && (offset % m.layout().align) != 0 {
            let padding_size = m.layout().align - (offset % m.layout().align);

            if padding_size > mem::size_of::<u64>() {
                let mut padding_fields = gen_padding_fields(ctx, paddings, offset, padding_size);

                fields.append(&mut padding_fields);

                paddings += padding_fields.len();
            }

            offset += padding_size;
        }

        debug!("member {}::{} @ {}, {:?}",
               name,
               m.name(),
               offset,
               m.layout());

        if let Some(f) = opt_f {
            let f_name = match f.bitfields {
                Some(_) => {
                    bitfields += 1;
                    format!("_bindgen_bitfield_{}_", bitfields)
                }
                None => rust_id(ctx, &f.name, &options.remove_prefix).0,
            };

            if !f.ty.can_auto_derive() {
                can_derive_debug = false;
                can_derive_clone = false;
            }

            let f_ty = P(cty_to_rs(ctx, &f.ty, options));

            fields.push(ast::StructField {
                span: ctx.span,
                vis: ast::Visibility::Public,
                ident: Some(ctx.ext_cx.ident_of(&f_name[..])),
                id: ast::DUMMY_NODE_ID,
                ty: f_ty,
                attrs: Vec::new(),
            });
        }

        if let Some(rc_c) = opt_rc_c {
            let c = rc_c.borrow();
            if c.name.is_empty() {
                unnamed += 1;
                let field_name = format!("_bindgen_data_{}_", unnamed);
                fields.push(mk_blob_field(ctx, &field_name, c.layout, ctx.span));
                methods.extend(gen_comp_methods(ctx,
                                                &field_name,
                                                0,
                                                c.kind,
                                                &c.members,
                                                &mut extra,
                                                options,
                                                derive_debug)
                                   .into_iter());
            } else {
                extra.extend(comp_to_rs(ctx,
                                        c.kind,
                                        comp_name(c.kind, &c.name),
                                        options,
                                        derive_debug,
                                        c.layout,
                                        c.members.clone())
                                 .into_iter());
            }
        }

        if let Some(rc_e) = opt_rc_e {
            let e = rc_e.borrow();
            assert!(!e.name.is_empty());
            extra.extend(cenum_to_rs(ctx,
                                     options,
                                     options.derive_debug,
                                     &enum_name(&e.name),
                                     e.kind,
                                     e.layout,
                                     &e.items));
        }

        offset += m.layout().size as usize;
    }

    if offset < layout.size {
        let mut padding_fields = gen_padding_fields(ctx, paddings, offset, layout.size - offset);

        fields.append(&mut padding_fields);
    }

    let def = ast::ItemKind::Struct(ast::VariantData::Struct(fields, ast::DUMMY_NODE_ID),
                                    ast::Generics::default());

    let id = rust_id(ctx, name, &options.remove_prefix).0;
    let mut attrs = vec![mk_repr_attr(ctx, layout)];
    if can_derive_clone {
        attrs.push(mk_attr(ctx, "derive", &["Copy", "Clone"]));
    } else {
        attrs.push(mk_attr(ctx, "derive", &["Copy"]));
    }
    if can_derive_debug {
        attrs.push(mk_deriving_debug_attr(ctx));
    }
    let struct_def = P(ast::Item {
        ident: ctx.ext_cx.ident_of(&id),
        attrs: attrs,
        id: ast::DUMMY_NODE_ID,
        node: def,
        vis: ast::Visibility::Public,
        span: ctx.span,
    });

    let mut items = vec![struct_def];
    if !methods.is_empty() {
        let impl_ = ast::ItemKind::Impl(ast::Unsafety::Normal,
                                        ast::ImplPolarity::Positive,
                                        ast::Generics::default(),
                                        None,
                                        P(mk_ty(ctx, false, vec![id.clone()])),
                                        methods);
        items.push(P(ast::Item {
            ident: ctx.ext_cx.ident_of(name),
            attrs: vec![],
            id: ast::DUMMY_NODE_ID,
            node: impl_,
            vis: ast::Visibility::Inherited,
            span: ctx.span,
        }));
    }

    if !can_derive_clone {
        items.push(mk_clone_impl(ctx, &id, options.use_core));
    }

    items.push(mk_default_impl(ctx, &id, options.use_core));
    items.extend(extra.into_iter());
    items
}

// Implements std::clone::Clone using dereferencing
fn mk_clone_impl(ctx: &GenCtx, ty_name: &str, use_core: bool) -> P<ast::Item> {
    let root_crate = if use_core {
        "core"
    } else {
        "std"
    };
    let impl_str = format!(r"
        impl ::{}::clone::Clone for {} {{
            fn clone(&self) -> Self {{ *self }}
        }}
    ",
                           root_crate,
                           ty_name);

    parse::new_parser_from_source_str(ctx.ext_cx.parse_sess(),
                                      ctx.ext_cx.cfg(),
                                      "".to_owned(),
                                      impl_str)
        .parse_item()
        .unwrap()
        .unwrap()
}

/// Convert a opaque type name to an ast Item.
fn opaque_to_rs(ctx: &mut GenCtx, name: &str, remove_prefix: &str) -> P<ast::Item> {
    let def = ast::ItemKind::Enum(ast::EnumDef { variants: vec![] }, ast::Generics::default());

    let id = rust_id(ctx, name, remove_prefix).0;
    P(ast::Item {
        ident: ctx.ext_cx.ident_of(&id),
        attrs: Vec::new(),
        id: ast::DUMMY_NODE_ID,
        node: def,
        vis: ast::Visibility::Public,
        span: ctx.span,
    })
}

fn cunion_to_rs(ctx: &mut GenCtx,
                name: String,
                options: &BindgenOptions,
                derive_debug: bool,
                layout: Layout,
                members: Vec<CompMember>)
                -> Vec<P<ast::Item>> {
    fn mk_item(ctx: &mut GenCtx,
               name: String,
               item: ast::ItemKind,
               vis: ast::Visibility,
               attrs: Vec<ast::Attribute>)
               -> P<ast::Item> {
        P(ast::Item {
            ident: ctx.ext_cx.ident_of(&name[..]),
            attrs: attrs,
            id: ast::DUMMY_NODE_ID,
            node: item,
            vis: vis,
            span: ctx.span,
        })
    }

    let ci = Rc::new(RefCell::new(CompInfo::new(name.clone(),
                                                CompKind::Union,
                                                members.clone(),
                                                layout)));
    let union = TNamed(Rc::new(RefCell::new(TypeInfo::new(name.clone(), TComp(ci), layout))));

    // Nested composites may need to emit declarations and implementations as
    // they are encountered.  The declarations end up in 'extra' and are emitted
    // after the current union.
    let mut extra = vec![];

    let data_field_name = "_bindgen_data_";
    let data_field = mk_blob_field(ctx, data_field_name, layout, ctx.span);

    let def = ast::ItemKind::Struct(ast::VariantData::Struct(vec![data_field], ast::DUMMY_NODE_ID),
                                    ast::Generics::default());
    let union_id = rust_id(ctx, &name, &options.remove_prefix).0;

    // Waiting for https://github.com/rust-lang/rfcs/issues/1038
    let can_auto_derive = members.iter()
                                 .all(|member| {
                                     match *member {
                                         CompMember::Field(ref f) |
                                         CompMember::CompField(_, ref f) => f.ty.can_auto_derive(),
                                         _ => true,
                                     }
                                 });
    let union_attrs = {
        let mut attrs = vec![mk_repr_attr(ctx, layout)];
        if can_auto_derive {
            attrs.push(mk_deriving_copy_clone_attr(ctx));
            if derive_debug {
                attrs.push(mk_deriving_debug_attr(ctx));
            }
        } else {
            attrs.push(mk_attr(ctx, "derive", &["Copy"]));
        }
        attrs
    };

    let union_def = mk_item(ctx,
                            union_id.clone(),
                            def,
                            ast::Visibility::Public,
                            union_attrs);

    let union_impl = ast::ItemKind::Impl(ast::Unsafety::Normal,
                                         ast::ImplPolarity::Positive,
                                         ast::Generics::default(),
                                         None,
                                         P(cty_to_rs(ctx, &union, options)),
                                         gen_comp_methods(ctx,
                                                          data_field_name,
                                                          0,
                                                          CompKind::Union,
                                                          &members,
                                                          &mut extra,
                                                          options,
                                                          derive_debug));

    let mut items = vec![union_def,
                         mk_item(ctx,
                                 "".to_owned(),
                                 union_impl,
                                 ast::Visibility::Inherited,
                                 Vec::new())];

    if !can_auto_derive {
        items.push(mk_clone_impl(ctx, &union_id, options.use_core));
    }

    items.push(mk_default_impl(ctx, &union_id, options.use_core));
    items.extend(extra.into_iter());
    items
}

fn i64_abs(i: i64) -> u64 {
    if i<0 {
        i.wrapping_neg() as u64
    } else {
        i as u64
    }
}

/// Converts a signed number to AST Expression.
fn i64_to_int_lit(ctx: &mut GenCtx, value: i64) -> P<ast::Expr> {
    let int_lit = ast::LitKind::Int(i64_abs(value), ast::LitIntType::Unsuffixed);
    let expr = ctx.ext_cx.expr_lit(ctx.span, int_lit);
    if value < 0 {
        let negated = ast::ExprKind::Unary(ast::UnOp::Neg, expr);
        ctx.ext_cx.expr(ctx.span, negated)
    } else {
        expr
    }
}

/// Converts a C const to Rust AST.
fn const_to_rs(ctx: &mut GenCtx,
               name: &str,
               val: i64,
               val_ty: ast::Ty,
               options: &BindgenOptions)
               -> P<ast::Item> {
    let int_lit = i64_to_int_lit(ctx, val);

    let cst = ast::ItemKind::Const(P(val_ty), int_lit);

    let id = rust_id(ctx, name, &options.remove_prefix).0;
    P(ast::Item {
        ident: ctx.ext_cx.ident_of(&id),
        attrs: Vec::new(),
        id: ast::DUMMY_NODE_ID,
        node: cst,
        vis: ast::Visibility::Public,
        span: ctx.span,
    })
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
        _ => unreachable!("invalid enum decl: signed: {}, size: {}", signed, size),
    }
}

fn enum_size_to_unsigned_max_value(size: usize) -> u64 {
    match size {
        1 => std::u8::MAX as u64,
        2 => std::u16::MAX as u64,
        4 => std::u32::MAX as u64,
        8 => std::u64::MAX,
        _ => unreachable!("invalid enum size: {}", size),
    }
}

/// Converts a C enum variant to an AST expression.
fn cenum_value_to_int_lit(ctx: &mut GenCtx,
                          enum_is_signed: bool,
                          size: usize,
                          value: i64)
                          -> P<ast::Expr> {
    if enum_is_signed {
        i64_to_int_lit(ctx, value)
    } else {
        let u64_value = value as u64 & enum_size_to_unsigned_max_value(size);
        let int_lit = ast::LitKind::Int(u64_value, ast::LitIntType::Unsuffixed);
        ctx.ext_cx.expr_lit(ctx.span, int_lit)
    }
}

fn cenum_to_rs(ctx: &mut GenCtx,
               options: &BindgenOptions,
               derive_debug: bool,
               name: &str,
               kind: IKind,
               layout: Layout,
               enum_items: &[EnumItem])
               -> Vec<P<ast::Item>> {
    let mangled_name = rust_id(ctx, name, &options.remove_prefix).0;
    let enum_name = ctx.ext_cx.ident_of(&mangled_name);
    let enum_ty = ctx.ext_cx.ty_ident(ctx.span, enum_name);
    let enum_is_signed = kind.is_signed();
    let enum_repr = enum_size_to_rust_type_name(enum_is_signed, layout.size);
    let mut items = vec![];

    if !options.rust_enums {
        items.push(ctx.ext_cx.item_ty(ctx.span,
                                      enum_name,
                                      ctx.ext_cx
                                         .ty_ident(ctx.span, ctx.ext_cx.ident_of(enum_repr)))
                   .map(|p|ast::Item{vis:ast::Visibility::Public,..p}));
        for item in enum_items {
            let rust_name = rust_id(ctx, &item.name, &options.remove_prefix).0;
            let value = cenum_value_to_int_lit(ctx, enum_is_signed, layout.size, item.val);
            items.push(ctx.ext_cx.item_const(ctx.span,
                                             ctx.ext_cx.ident_of(&rust_name),
                                             enum_ty.clone(),
                                             value)
                       .map(|p|ast::Item{vis:ast::Visibility::Public,..p}));
        }
        return items;
    }

    let mut variants = vec![];
    let mut found_values = HashMap::new();

    for item in enum_items {
        let rust_name = rust_id(ctx, &item.name, &options.remove_prefix).0;
        let name = ctx.ext_cx.ident_of(&rust_name);

        if let Some(orig) = found_values.get(&item.val) {
            let value = ctx.ext_cx.expr_path(ctx.ext_cx.path(ctx.span, vec![enum_name, *orig]));
            // Can't use ctx.ext_cx.item because of Visibility::Public
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

        let value = cenum_value_to_int_lit(ctx, enum_is_signed, layout.size, item.val);

        variants.push(respan(ctx.span,
                             ast::Variant_ {
                                 name: name,
                                 attrs: vec![],
                                 data: ast::VariantData::Unit(ast::DUMMY_NODE_ID),
                                 disr_expr: Some(value),
                             }));
    }

    let repr_attr = mk_attr(ctx, "repr", &[enum_repr]);

    let attrs = {
        let mut v = vec![mk_deriving_copy_clone_attr(ctx), repr_attr];
        if derive_debug {
            v.push(mk_deriving_debug_attr(ctx));
        }
        v
    };

    items.push(P(ast::Item {
        ident: enum_name,
        attrs: attrs,
        id: ast::DUMMY_NODE_ID,
        node: ast::ItemKind::Enum(ast::EnumDef { variants: variants },
                                  ast::Generics::default()),
        vis: ast::Visibility::Public,
        span: ctx.span,
    }));

    items
}

/// Generates accessors for fields in nested structs and unions which must be
/// represented in Rust as an untyped array.  This process may generate
/// declarations and implementations that must be placed at the root level.
/// These are emitted into `extra`.
#[cfg_attr(feature = "clippy", allow(too_many_arguments))]
fn gen_comp_methods(ctx: &mut GenCtx,
                    data_field: &str,
                    data_offset: usize,
                    kind: CompKind,
                    members: &[CompMember],
                    extra: &mut Vec<P<ast::Item>>,
                    options: &BindgenOptions,
                    derive_debug: bool)
                    -> Vec<ast::ImplItem> {

    let mk_field_method = |ctx: &mut GenCtx, f: &FieldInfo, offset: usize| {
        // TODO: Implement bitfield accessors
        if f.bitfields.is_some() {
            return None;
        }

        let (f_name, _) = rust_id(ctx, &f.name, &options.remove_prefix);
        let ret_ty = P(cty_to_rs(ctx,
                                 &TPtr(Box::new(f.ty.clone()), false, Layout::default()),
                                 options));

        // When the offset is zero, generate slightly prettier code.
        let method = {
            let root_crate = if options.use_core {
                "core"
            } else {
                "std"
            };
            let impl_str = format!(r"
                impl X {{
                    pub unsafe fn {}(&mut self) -> {} {{
                        let raw: *mut u8 = ::{root_crate}::mem::transmute(&self.{});
                        ::{root_crate}::mem::transmute(raw.offset({}))
                    }}
                }}
            ",
                                   f_name,
                                   tts_to_string(&ret_ty.to_tokens(&ctx.ext_cx)[..]),
                                   data_field,
                                   offset,
                                   root_crate = root_crate);

            parse::new_parser_from_source_str(ctx.ext_cx.parse_sess(),
                                              ctx.ext_cx.cfg(),
                                              "".to_owned(),
                                              impl_str)
                .parse_item()
                .unwrap()
                .unwrap()
        };

        method.and_then(|i| {
            match i.node {
                ast::ItemKind::Impl(_, _, _, _, _, mut items) => items.pop(),
                _ => unreachable!("impl parsed to something other than impl"),
            }
        })
    };

    let mut offset = data_offset;
    let mut methods = vec![];
    for m in members.into_iter() {
        let advance_by = match *m {
            CompMember::Field(ref f) => {
                methods.extend(mk_field_method(ctx, f, offset).into_iter());
                f.ty.size()
            }
            CompMember::Comp(ref rc_c) => {
                let c = &rc_c.borrow();
                methods.extend(gen_comp_methods(ctx,
                                                data_field,
                                                offset,
                                                c.kind,
                                                &c.members,
                                                extra,
                                                options,
                                                derive_debug)
                                   .into_iter());
                c.layout.size
            }
            CompMember::CompField(ref rc_c, ref f) => {
                methods.extend(mk_field_method(ctx, f, offset).into_iter());

                let c = rc_c.borrow();
                extra.extend(comp_to_rs(ctx,
                                        c.kind,
                                        comp_name(c.kind, &c.name),
                                        options,
                                        derive_debug,
                                        c.layout,
                                        c.members.clone())
                                 .into_iter());
                f.ty.size()
            }
            CompMember::Enum(ref rc_e) => rc_e.borrow().layout.size,
            CompMember::EnumField(ref _rc_e, ref f) => f.ty.size(),
        };
        match kind {
            CompKind::Struct => {
                offset += advance_by;
            }
            CompKind::Union => {}
        }
    }
    methods
}

// Implements std::default::Default using std::mem::zeroed.
fn mk_default_impl(ctx: &GenCtx, ty_name: &str, use_core: bool) -> P<ast::Item> {
    let root_crate = if use_core {
        "core"
    } else {
        "std"
    };
    let impl_str = format!(r"
        impl ::{root_crate}::default::Default for {} {{
            fn default() -> Self {{ unsafe {{ ::{root_crate}::mem::zeroed() }} }}
        }}
    ",
                           ty_name,
                           root_crate = root_crate);

    parse::new_parser_from_source_str(ctx.ext_cx.parse_sess(),
                                      ctx.ext_cx.cfg(),
                                      "".to_owned(),
                                      impl_str)
        .parse_item()
        .unwrap()
        .unwrap()
}

fn mk_blob_field(ctx: &GenCtx, name: &str, layout: Layout, span: Span) -> ast::StructField {
    let ty_name = match layout.align {
        8 => "u64",
        4 => "u32",
        2 => "u16",
        1 | _ => "u8",
    };
    let data_len = if ty_name == "u8" {
        layout.size
    } else {
        layout.size / layout.align
    };
    let base_ty = mk_ty(ctx, false, vec![ty_name.to_owned()]);
    let data_ty = P(mk_arrty(ctx, &base_ty, data_len));
    ast::StructField {
        span: span,
        vis: ast::Visibility::Public,
        ident: Some(ctx.ext_cx.ident_of(name)),
        id: ast::DUMMY_NODE_ID,
        ty: data_ty,
        attrs: Vec::new(),
    }
}

fn mk_link_name_attr(ctx: &mut GenCtx, name: &str) -> ast::Attribute {
    let attr = {
        let k = ctx.ext_cx.name_of("link_name").as_str();
        let v = ctx.ext_cx.name_of(name).as_str();
        ctx.ext_cx.meta_name_value(ctx.span, k, ast::LitKind::Str(v,ast::StrStyle::Cooked))
    };
    respan(ctx.span,
           ast::Attribute_ {
               id: mk_attr_id(),
               style: ast::AttrStyle::Outer,
               value: attr,
               is_sugared_doc: false,
           })
}

fn mk_repr_attr(ctx: &mut GenCtx, layout: Layout) -> ast::Attribute {
    let mut values = vec!["C"];
    if layout.packed {
        values.push("packed");
    }
    mk_attr(ctx, "repr", &values)
}

fn mk_deriving_copy_clone_attr(ctx: &mut GenCtx) -> ast::Attribute {
    mk_attr(ctx, "derive", &["Copy", "Clone"])
}

fn mk_deriving_debug_attr(ctx: &mut GenCtx) -> ast::Attribute {
    mk_attr(ctx, "derive", &["Debug"])
}


fn mk_attr(ctx: &mut GenCtx, name: &str, args: &[&str]) -> ast::Attribute {
    mk_attr_style(ctx, name, args, ast::AttrStyle::Outer)
}

fn mk_attr_style(ctx: &mut GenCtx,
                 name: &str,
                 args: &[&str],
                 style: ast::AttrStyle)
                 -> ast::Attribute {
    let args: Vec<_> = args.iter()
                           .map(|arg| {
                               let word = ctx.ext_cx.name_of(arg).as_str();
                               ctx.ext_cx.meta_word(ctx.span, word)
                           })
                           .collect();
    let attr = {
        let name = ctx.ext_cx.name_of(name).as_str();
        ctx.ext_cx.meta_list(ctx.span, name, args)
    };
    respan(ctx.span,
           ast::Attribute_ {
               id: mk_attr_id(),
               style: style,
               value: attr,
               is_sugared_doc: false,
           })
}

fn cvar_to_rs(ctx: &mut GenCtx,
              name: String,
              ty: &Type,
              is_const: bool,
              options: &BindgenOptions)
              -> ast::ForeignItem {
    let (rust_name, was_mangled) = rust_id(ctx, &name, &options.remove_prefix);

    let mut attrs = Vec::new();
    if was_mangled {
        attrs.push(mk_link_name_attr(ctx, &name));
    }

    let node = {
        let val_ty = P(cty_to_rs(ctx, ty, options));
        ast::ForeignItemKind::Static(val_ty, !is_const)
    };

    mk_foreign_item(ctx, &rust_name, attrs, node)
}

fn mk_foreign_item(ctx: &mut GenCtx,
                   name: &str,
                   attrs: Vec<ast::Attribute>,
                   node: ast::ForeignItemKind)
                   -> ast::ForeignItem {
    ast::ForeignItem {
        ident: ctx.ext_cx.ident_of(name),
        attrs: attrs,
        node: node,
        id: ast::DUMMY_NODE_ID,
        span: ctx.span,
        vis: ast::Visibility::Public,
    }
}

fn cfuncty_to_rs(ctx: &mut GenCtx,
                 rty: &Type,
                 aty: &[(String, Type)],
                 var: bool,
                 options: &BindgenOptions)
                 -> ast::FnDecl {

    let ret = match *rty {
        TVoid => ast::FunctionRetTy::Default(ctx.span),
        _ => ast::FunctionRetTy::Ty(P(cty_to_rs(ctx, rty, options))),
    };

    let mut unnamed: usize = 0;
    let args: Vec<ast::Arg> =
        aty.iter()
           .map(|arg| {
               let (ref n, ref t) = *arg;

               let arg_name = if n.is_empty() {
                   unnamed += 1;
                   format!("arg{}", unnamed)
               } else {
                   rust_id(ctx, n, &options.remove_prefix).0
               };

               // From the C90 standard (http://c0x.coding-guidelines.com/6.7.5.3.html)
               // 1598 - A declaration of a parameter as “array of type” shall be
               // adjusted to “qualified pointer to type”, where the type qualifiers
               // (if any) are those specified within the [ and ] of the array type
               // derivation.
               let arg_ty = P(match *t {
                   TArray(ref typ, _, l) => cty_to_rs(ctx, &TPtr(typ.clone(), false, l), options),
                   _ => cty_to_rs(ctx, t, options),
               });
               let ident = ctx.ext_cx.ident_of(&arg_name);

               ctx.ext_cx.arg(ctx.span, ident, arg_ty)
           })
           .collect();

    let var = !args.is_empty() && var;
    ast::FnDecl {
        inputs: args,
        output: ret,
        variadic: var,
    }
}

fn cfunc_to_rs(ctx: &mut GenCtx,
               name: String,
               rty: &Type,
               aty: &[(String, Type)],
               var: bool,
               options: &BindgenOptions)
               -> ast::ForeignItem {
    let var = !aty.is_empty() && var;
    let decl = ast::ForeignItemKind::Fn(P(cfuncty_to_rs(ctx, rty, aty, var, options)),
                                        ast::Generics::default());

    let (rust_name, was_mangled) = rust_id(ctx, &name, &options.remove_prefix);

    let mut attrs = Vec::new();
    if was_mangled {
        attrs.push(mk_link_name_attr(ctx, &name));
    }

    mk_foreign_item(ctx, &rust_name, attrs, decl)
}

fn is_named_fnproto(ty: &Type) -> bool {
    if let &TNamed(ref rc)=ty {
        if let TFuncProto(..)=rc.borrow().ty {
            return true
        }
    }
    false
}

fn cty_to_rs(ctx: &mut GenCtx, ty: &Type, options: &BindgenOptions) -> ast::Ty {
    let raw = |fragment: &str| {
        let mut path = options.ctypes_prefix.clone();
        path.push(fragment.to_owned());
        path
    };

    match *ty {
        TVoid => mk_ty(ctx, true, raw("c_void")),
        TInt(i, ref layout) => {
            match i {
                IBool => {
                    let ty_name = match layout.size {
                        8 => "u64",
                        4 => "u32",
                        2 => "u16",
                        1 | _ => "u8",
                    };
                    mk_ty(ctx, false, vec![ty_name.to_owned()])
                }
                ISChar => mk_ty(ctx, true, raw("c_char")),
                IUChar => mk_ty(ctx, true, raw("c_uchar")),
                IInt => mk_ty(ctx, true, raw("c_int")),
                IUInt => mk_ty(ctx, true, raw("c_uint")),
                IShort => mk_ty(ctx, true, raw("c_short")),
                IUShort => mk_ty(ctx, true, raw("c_ushort")),
                ILong => mk_ty(ctx, true, raw("c_long")),
                IULong => mk_ty(ctx, true, raw("c_ulong")),
                ILongLong => mk_ty(ctx, true, raw("c_longlong")),
                IULongLong => mk_ty(ctx, true, raw("c_ulonglong")),
                IWChar => mk_ty(ctx, true, raw("wchar_t")),
            }
        }
        TFloat(f, _) => {
            match f {
                FFloat => {
                    mk_ty(ctx,
                          !options.convert_floats,
                          if options.convert_floats {
                              vec!["f32".into()]
                          } else {
                              raw("c_float")
                          })
                }
                FDouble => {
                    mk_ty(ctx,
                          !options.convert_floats,
                          if options.convert_floats {
                              vec!["f64".into()]
                          } else {
                              raw("c_double")
                          })
                }
            }
        }
        TPtr(ref t, is_const, _) => {
            let id = cty_to_rs(ctx, &**t, options);
            if is_named_fnproto(&**t) {
                id
            } else {
                mk_ptrty(ctx, id, is_const)
            }
        }
        TArray(ref t, s, _) => {
            let ty = cty_to_rs(ctx, &**t, options);
            mk_arrty(ctx, &ty, s)
        }
        TFuncPtr(ref sig, _) | TFuncProto(ref sig, _) => {
            let decl = cfuncty_to_rs(ctx, &*sig.ret_ty, &sig.args[..], sig.is_variadic, options);
            let unsafety = if sig.is_safe {
                ast::Unsafety::Normal
            } else {
                ast::Unsafety::Unsafe
            };
            mk_fnty(ctx, decl, unsafety, sig.abi, options.use_core)
        }
        TNamed(ref ti) => {
            let id = rust_id(ctx, &ti.borrow().name, &options.remove_prefix).0;
            mk_ty(ctx, false, vec![id])
        }
        TComp(ref ci) => {
            let mut c = ci.borrow_mut();
            c.name = unnamed_name(ctx, &c.name);
            c.name = rust_id(ctx, &c.name, &options.remove_prefix).0;
            mk_ty(ctx, false, vec![comp_name(c.kind, &c.name)])
        }
        TEnum(ref ei) => {
            let mut e = ei.borrow_mut();
            e.name = unnamed_name(ctx, &e.name);
            e.name = rust_id(ctx, &e.name, &options.remove_prefix).0;
            mk_ty(ctx, false, vec![enum_name(&e.name)])
        }
    }
}

fn mk_ty(ctx: &GenCtx, global: bool, segments: Vec<String>) -> ast::Ty {
    let ty = ast::TyKind::Path(
        None,
        ast::Path {
            span: ctx.span,
            global: global,
            segments: segments.iter().map(|s| {
                ast::PathSegment {
                    identifier: ctx.ext_cx.ident_of(&s[..]),
                    parameters: ast::PathParameters::AngleBracketed(ast::AngleBracketedParameterData {
                        lifetimes: Vec::new(),
                        types: P::new(),
                        bindings: P::new(),
                    }),
                }
            }).collect()
        },
    );

    ctx.ext_cx.ty(ctx.span, ty).unwrap()
}

fn mk_ptrty(ctx: &mut GenCtx, base: ast::Ty, is_const: bool) -> ast::Ty {
    let mutability = if is_const {
        ast::Mutability::Immutable
    } else {
        ast::Mutability::Mutable
    };

    ctx.ext_cx.ty_ptr(ctx.span, P(base), mutability).unwrap()
}

fn mk_arrty(ctx: &GenCtx, base: &ast::Ty, n: usize) -> ast::Ty {
    let int_lit = ast::LitKind::Int(n as u64, ast::LitIntType::Unsigned(ast::UintTy::Us));
    let sz = ctx.ext_cx.expr_lit(ctx.span, int_lit).unwrap();
    let ty = ast::TyKind::FixedLengthVec(P(base.clone()), P(sz));

    ctx.ext_cx.ty(ctx.span, ty).unwrap()
}

fn mk_fnty(ctx: &mut GenCtx,
           decl: ast::FnDecl,
           unsafety: ast::Unsafety,
           abi: abi::Abi,
           use_core: bool)
           -> ast::Ty {
    let fnty = ast::TyKind::BareFn(P(ast::BareFnTy {
        unsafety: unsafety,
        abi: abi,
        lifetimes: Vec::new(),
        decl: P(decl),
    }));

    let idents = [if use_core {
                      "core"
                  } else {
                      "std"
                  },
                  "option",
                  "Option"]
                     .iter()
                     .map(|item| ctx.ext_cx.ident_of(item))
                     .collect();
    let types = vec![ctx.ext_cx.ty(ctx.span, fnty)];
    ctx.ext_cx
       .ty_path(ctx.ext_cx.path_all(ctx.span, true, idents, Vec::new(), types, Vec::new()))
       .unwrap()
}
