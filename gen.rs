#![allow(unused_must_use)]

use std::cell::RefCell;
use std::option;
use std::iter;
use std::vec::Vec;

use syntax::abi;
use syntax::ast;
use syntax::codemap::{Span, respan, ExpnInfo, NameAndSpan, MacroBang};
use syntax::ext::base;
use syntax::ext::build::AstBuilder;
use syntax::ext::expand::ExpansionConfig;
use syntax::owned_slice::OwnedSlice;
use syntax::parse;

use types::*;

struct GenCtx<'r> {
    ext_cx: base::ExtCtxt<'r>,
    unnamed_ty: uint,
    abi: abi::Abi,
    span: Span
}

struct ErrLoader;

impl base::CrateLoader for ErrLoader {
    fn load_crate(&mut self, _: &ast::ViewItem) -> base::MacroCrate {
        fail!("lolwut")
    }
}

fn first<A, B>((val, _): (A, B)) -> A {
    return val;
}

fn ref_eq<'a, 'b, T>(thing: &'a T, other: &'b T) -> bool {
    (thing as *T) == (other as *T)
}

fn to_intern_str(ctx: &mut GenCtx, s: StrBuf) -> parse::token::InternedString {
    let id = ctx.ext_cx.ident_of(s.as_slice());
    parse::token::get_ident(id)
}

fn empty_generics() -> ast::Generics {
    ast::Generics {
        lifetimes: Vec::new(),
        ty_params: OwnedSlice::empty(),
    }
}

fn rust_id(ctx: &mut GenCtx, name: StrBuf) -> (StrBuf, bool) {
    let token = parse::token::IDENT(ctx.ext_cx.ident_of(name.as_slice()), false);
    if parse::token::is_any_keyword(&token) || "bool" == name.as_slice() {
        ("_".to_owned().append(name.as_slice()), true)
    } else {
        (name, false)
    }

}

fn rust_type_id(ctx: &mut GenCtx, name: StrBuf) -> StrBuf {
    if "bool" == name.as_slice() ||
        "uint" == name.as_slice() ||
        "u8" == name.as_slice() ||
        "u16" == name.as_slice() ||
        "u32" == name.as_slice() ||
        "f32" == name.as_slice() ||
        "f64" == name.as_slice() ||
        "i8" == name.as_slice() ||
        "i16" == name.as_slice() ||
        "i32" == name.as_slice() ||
        "i64" == name.as_slice() ||
        "Self" == name.as_slice() ||
        "str" == name.as_slice() {
        "_".to_owned().append(name.as_slice())
    } else {
        let (n, _) = rust_id(ctx, name);
        n
    }
}

fn unnamed_name(ctx: &mut GenCtx, name: StrBuf) -> StrBuf {
    return if name.is_empty() {
        ctx.unnamed_ty += 1;
        format!("Unnamed{}", ctx.unnamed_ty)
    } else {
        name
    };
}

fn struct_name(name: StrBuf) -> StrBuf {
    format!("Struct_{}", name)
}

fn union_name(name: StrBuf) -> StrBuf {
    format!("Union_{}", name)
}

fn enum_name(name: StrBuf) -> StrBuf {
    format!("Enum_{}", name)
}

pub fn gen_mod(abi: &str, links: &[(StrBuf, Option<StrBuf>)], globs: Vec<Global>, span: Span) -> Vec<@ast::Item> {
    let abi = match abi {
        "cdecl" => abi::Cdecl,
        "stdcall" => abi::Stdcall,
        "fastcall" => abi::Fastcall,
        "aapcs" => abi::Aapcs,
        "Rust" => abi::Rust,
        "rust-intrinsic" => abi::RustIntrinsic,
        _ => abi::C
    };

    // Create a dummy ExtCtxt. We only need this for string interning and that uses TLS.
    let mut loader = ErrLoader;
    let cfg = ExpansionConfig {
        loader: &mut loader,
        deriving_hash_type_parameter: false,
        crate_id: from_str("xxx").unwrap(),
    };
    let sess = &parse::new_parse_sess();
    let mut ctx = GenCtx {
        ext_cx: base::ExtCtxt::new(
            sess,
            Vec::new(),
            cfg,
        ),
        unnamed_ty: 0,
        abi: abi,
        span: span
    };
    ctx.ext_cx.bt_push(ExpnInfo {
        call_site: ctx.span,
        callee: NameAndSpan { name: StrBuf::new(), format: MacroBang, span: None }
    });
    let uniq_globs = tag_dup_decl(globs);

    let mut fs = vec!();
    let mut vs = vec!();
    let mut gs = vec!();
    for g in uniq_globs.move_iter() {
        match g {
            GOther => {}
            GFunc(_) => fs.push(g),
            GVar(_) => vs.push(g),
            _ => gs.push(g)
        }
    }

    let mut defs = vec!();
    gs = remove_redundant_decl(gs);

    for g in gs.move_iter() {
        match g {
            GType(ti) => {
                let t = ti.borrow().clone();
                defs.push_all_move(ctypedef_to_rs(&mut ctx, t.name.clone(), &t.ty))
            },
            GCompDecl(ci) => {
                {
                    let mut c = ci.borrow_mut();
                    c.name = unnamed_name(&mut ctx, c.name.clone());
                }
                let c = ci.borrow().clone();
                if c.cstruct {
                    defs.push_all_move(ctypedef_to_rs(&mut ctx, struct_name(c.name), &TVoid))
                } else {
                    defs.push_all_move(ctypedef_to_rs(&mut ctx, union_name(c.name), &TVoid))
                }
            },
            GComp(ci) => {
                {
                    let mut c = ci.borrow_mut();
                    c.name = unnamed_name(&mut ctx, c.name.clone());
                }
                let c = ci.borrow().clone();
                if c.cstruct {
                    defs.push(cstruct_to_rs(&mut ctx, struct_name(c.name.clone()),
                                            // this clone is necessary to prevent dynamic borrow
                                            // check errors.
                                            // FIXME: remove the @mut in types.rs to fix this
                                            c.fields.clone()))
                } else {
                    defs.push_all_move(cunion_to_rs(&mut ctx, union_name(c.name.clone()),
                                               c.layout, c.fields))
                }
            },
            GEnumDecl(ei) => {
                {
                    let mut e = ei.borrow_mut();
                    e.name = unnamed_name(&mut ctx, e.name.clone());
                }
                let e = ei.borrow().clone();
                defs.push_all_move(ctypedef_to_rs(&mut ctx, enum_name(e.name.clone()), &TVoid))
            },
            GEnum(ei) => {
                {
                    let mut e = ei.borrow_mut();
                    e.name = unnamed_name(&mut ctx, e.name.clone());
                }
                let e = ei.borrow().clone();
                defs.push_all_move(cenum_to_rs(&mut ctx, enum_name(e.name.clone()), e.kind, e.items))
            },
            _ => { }
        }
    }

    let vars = vs.move_iter().map(|v| {
        match v {
            GVar(vi) => {
                let v = vi.borrow();
                cvar_to_rs(&mut ctx, v.name.clone(), &v.ty, v.is_const)
            },
            _ => unreachable!()
        }
    }).collect();

    let funcs = fs.move_iter().map(|f| {
        match f {
            GFunc(vi) => {
                let v = vi.borrow();
                match v.ty {
                    TFunc(ref rty, ref aty, var) =>
                        cfunc_to_rs(&mut ctx, v.name.clone(),
                                    *rty, aty.as_slice(), var),
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }).collect();

    defs.push(mk_extern(&mut ctx, links, vars, funcs));
    
    //let attrs = vec!(mk_attr_list(&mut ctx, "allow", ["dead_code", "non_camel_case_types", "uppercase_variables"]));
    
    defs
}

fn mk_attr_list(ctx: &mut GenCtx, attr_name: &str, items: &[&str]) -> ast::Attribute {
    let items: Vec<_> = items.iter().map(|item| {
        let interned = to_intern_str(ctx, item.to_owned());
        ctx.ext_cx.meta_word(ctx.span, interned)
    }).collect();
    let interned = to_intern_str(ctx, attr_name.to_owned());
    ctx.ext_cx.attribute(ctx.span, ctx.ext_cx.meta_list(ctx.span, interned, items))
}

fn mk_import(ctx: &mut GenCtx, path: &[StrBuf]) -> ast::ViewItem {
    let view = ast::ViewItemUse(
        @respan(ctx.span,
            ast::ViewPathGlob(
                ast::Path {
                    span: ctx.span,
                    global: false,
                    segments: path.iter().map(|p|
                        ast::PathSegment {
                            identifier: ctx.ext_cx.ident_of((*p).as_slice()),
                            lifetimes: Vec::new(),
                            types: OwnedSlice::empty(),
                        }
                    ).collect()
                },
                ast::DUMMY_NODE_ID
            )
        )
    );

    return ast::ViewItem {
              node: view,
              attrs: Vec::new(),
              vis: ast::Inherited,
              span: ctx.span
           };
}

fn mk_extern(ctx: &mut GenCtx, links: &[(StrBuf, Option<StrBuf>)],
             vars: Vec<@ast::ForeignItem>,
             funcs: Vec<@ast::ForeignItem>) -> @ast::Item {
    let attrs = if links.is_empty() {
        Vec::new()
    } else {
        links.iter().map(|&(ref l, ref k)| {
            let link_name = @respan(ctx.span, ast::MetaNameValue(
                to_intern_str(ctx, "name".to_owned()),
                respan(ctx.span, ast::LitStr(
                    to_intern_str(ctx, l.to_owned()),
                    ast::CookedStr
                ))
            ));
            let link_args = match k {
                &None => vec!(link_name),
                &Some(ref k) => vec!(link_name, @respan(ctx.span, ast::MetaNameValue(
                    to_intern_str(ctx, "kind".to_owned()),
                    respan(ctx.span, ast::LitStr(
                        to_intern_str(ctx, k.to_owned()),
                        ast::CookedStr
                    ))
                )))
            };
            respan(ctx.span, ast::Attribute_ {
                style: ast::AttrOuter,
                value: @respan(ctx.span, ast::MetaList(
                    to_intern_str(ctx, "link".to_owned()),
                    link_args)
                ),
                is_sugared_doc: false
            })
        }).collect()
    };

    let mut items = Vec::new();
    items.push_all_move(vars);
    items.push_all_move(funcs);
    let ext = ast::ItemForeignMod(ast::ForeignMod {
        abi: ctx.abi,
        view_items: Vec::new(),
        items: items
    });

    return @ast::Item {
              ident: ctx.ext_cx.ident_of(""),
              attrs: attrs,
              id: ast::DUMMY_NODE_ID,
              node: ext,
              vis: ast::Inherited,
              span: ctx.span
           };
}

fn remove_redundant_decl(gs: Vec<Global>) -> Vec<Global> {
    fn check_decl(a: &Global, ty: &Type) -> bool {
        match *a {
          GComp(ci1) => match *ty {
              TComp(ci2) => {
                  ref_eq(ci1, ci2) && ci1.borrow().name.is_empty()
              },
              _ => false
          },
          GEnum(ei1) => match *ty {
              TEnum(ei2) => {
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

    return gs.move_iter().filter(|g|
        !typedefs.iter().any(|t| check_decl(g, t))
    ).collect();
}

fn tag_dup_decl(gs: Vec<Global>) -> Vec<Global> {
    fn check(name1: &str, name2: &str) -> bool {
        !name1.is_empty() && name1 == name2
    }

    fn check_dup(g1: &Global, g2: &Global) -> bool {
        match (g1, g2) {
          (&GType(ti1), &GType(ti2)) => {
              let a = ti1.borrow();
              let b = ti2.borrow();
              check(a.name.as_slice(), b.name.as_slice())
          },
          (&GComp(ci1), &GComp(ci2)) => {
              let a = ci1.borrow();
              let b = ci2.borrow();
              check(a.name.as_slice(), b.name.as_slice())
          },
          (&GCompDecl(ci1), &GCompDecl(ci2)) => {
              let a = ci1.borrow();
              let b = ci2.borrow();
              check(a.name.as_slice(), b.name.as_slice())
          },
          (&GEnum(ei1), &GEnum(ei2)) => {
              let a = ei1.borrow();
              let b = ei2.borrow();
              check(a.name.as_slice(), b.name.as_slice())
          },
          (&GEnumDecl(ei1), &GEnumDecl(ei2)) => {
              let a = ei1.borrow();
              let b = ei2.borrow();
              check(a.name.as_slice(), b.name.as_slice())
          },
          (&GVar(vi1), &GVar(vi2)) => {
              let a = vi1.borrow();
              let b = vi2.borrow();
              check(a.name.as_slice(), b.name.as_slice())
          },
          (&GFunc(vi1), &GFunc(vi2)) => {
              let a = vi1.borrow();
              let b = vi2.borrow();
              check(a.name.as_slice(), b.name.as_slice())
          },
          _ => false
        }
    }

    if gs.is_empty() {
        return gs;
    }

    let len = gs.len();
    let mut res: Vec<Global> = vec!();
    res.push(*gs.get(0));

    for i in iter::range(1, len) {
        let mut dup = false;
        for j in iter::range(0, i-1) {
            if check_dup(gs.get(i), gs.get(j)) {
                dup = true;
                break;
            }
        }
        if !dup {
            res.push(*gs.get(i));
        }
    }

    return res;
}

fn ctypedef_to_rs(ctx: &mut GenCtx, name: StrBuf, ty: &Type) -> Vec<@ast::Item> {
    fn mk_item(ctx: &mut GenCtx, name: StrBuf, ty: &Type) -> @ast::Item {
        let rust_name = rust_type_id(ctx, name);
        let rust_ty = cty_to_rs(ctx, ty);
        let base = ast::ItemTy(
            @ast::Ty {
                id: ast::DUMMY_NODE_ID,
                node: rust_ty.node,
                span: ctx.span,
            },
            empty_generics()
        );

        return @ast::Item {
                  ident: ctx.ext_cx.ident_of(rust_name.as_slice()),
                  attrs: Vec::new(),
                  id: ast::DUMMY_NODE_ID,
                  node: base,
                  vis: ast::Public,
                  span: ctx.span
               };
    }

    return match *ty {
        TComp(ci) => {
            let is_empty = ci.borrow().name.is_empty();
            if is_empty {
                ci.borrow_mut().name = name.clone();
                let c = ci.borrow().clone();
                if c.cstruct {
                    vec!(cstruct_to_rs(ctx, name, c.fields))
                } else {
                    cunion_to_rs(ctx, name, c.layout, c.fields)
                }
            } else {
                vec!(mk_item(ctx, name, ty))
            }
        },
        TEnum(ei) => {
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

fn cstruct_to_rs(ctx: &mut GenCtx, name: StrBuf, fields: Vec<FieldInfo>) -> @ast::Item {
    let mut unnamed = 0;
    let fs = fields.iter().map(|f| {
        let f_name = if f.name.is_empty() || "_" == f.name.as_slice() {
            unnamed += 1;
            format!("unnamed_field{}", unnamed)
        } else {
            rust_type_id(ctx, f.name.clone())
        };

        let f_ty = @cty_to_rs(ctx, &f.ty);

        respan(ctx.span, ast::StructField_ {
            kind: ast::NamedField(
                ctx.ext_cx.ident_of(f_name.as_slice()),
                ast::Public,
            ),
            id: ast::DUMMY_NODE_ID,
            ty: f_ty,
            attrs: Vec::new()
        })
    }).collect();

    let def = ast::ItemStruct(
        @ast::StructDef {
           fields: fs,
           ctor_id: None,
           super_struct: None,
           is_virtual: false
        },
        empty_generics()
    );

    let id = rust_type_id(ctx, name);
    return @ast::Item { ident: ctx.ext_cx.ident_of(id.as_slice()),
              attrs: Vec::new(),
              id: ast::DUMMY_NODE_ID,
              node: def,
              vis: ast::Public,
              span: ctx.span
           };
}

fn cunion_to_rs(ctx: &mut GenCtx, name: StrBuf, layout: Layout, fields: Vec<FieldInfo>) -> Vec<@ast::Item> {
    fn mk_item(ctx: &mut GenCtx, name: StrBuf, item: ast::Item_, vis: ast::Visibility) -> @ast::Item {
        return @ast::Item {
                  ident: ctx.ext_cx.ident_of(name.as_slice()),
                  attrs: Vec::new(),
                  id: ast::DUMMY_NODE_ID,
                  node: item,
                  vis: vis,
                  span: ctx.span
               };
    }

    let ci = @RefCell::new(CompInfo::new(name.clone(), false, fields.clone(), layout));
    let union = TNamed(@RefCell::new(TypeInfo::new(name.clone(), TComp(ci))));

    let ty_name = match layout.align {
        1 => "u8",
        2 => "u16",
        4 => "u32",
        8 => "u64",
        _ => "u8",
    };
    let data_len = if ty_name == "u8" { layout.size } else { layout.size / layout.align };
    let base_ty = mk_ty(ctx, false, vec!(ty_name.to_owned()));
    let data_ty = @mk_arrty(ctx, &base_ty, data_len);
    let data = respan(ctx.span, ast::StructField_ {
        kind: ast::NamedField(
            ctx.ext_cx.ident_of("data"),
            ast::Public,
        ),
        id: ast::DUMMY_NODE_ID,
        ty: data_ty,
        attrs: Vec::new()
    });

    let def = ast::ItemStruct(
        @ast::StructDef {
           fields: Vec::from_elem(1, data),
           ctor_id: None,
           super_struct: None,
           is_virtual: false
        },
        empty_generics()
    );
    let union_id = rust_type_id(ctx, name);
    let union_def = mk_item(ctx, union_id, def, ast::Public);

    let expr = quote_expr!(
        &ctx.ext_cx,
        unsafe { ::std::mem::transmute(self) }
    );
    let mut unnamed = 0;
    let fs = fields.iter().map(|f| {
        let f_name = if f.name.is_empty() || "_" == f.name.as_slice() {
            unnamed += 1;
            format!("unnamed_field{}", unnamed)
        } else {
            first(rust_id(ctx, f.name.clone()))
        };

        let ret_ty = @cty_to_rs(ctx, &TPtr(box f.ty.clone(), false, Layout::zero()));
        let body = @ast::Block {
            view_items: Vec::new(),
            stmts: Vec::new(),
            expr: Some(expr),
            id: ast::DUMMY_NODE_ID,
            rules: ast::DefaultBlock,
            span: ctx.span
        };

        @ast::Method {
            ident: ctx.ext_cx.ident_of(f_name.as_slice()),
            attrs: Vec::new(),
            generics: empty_generics(),
            explicit_self: respan(ctx.span, ast::SelfRegion(None, ast::MutMutable)),
            fn_style: ast::NormalFn,
            decl: @ast::FnDecl {
                inputs: Vec::from_elem(1, ast::Arg::new_self(ctx.span, ast::MutImmutable)),
                output: ret_ty,
                cf: ast::Return,
                variadic: false
            },
            body: body,
            id: ast::DUMMY_NODE_ID,
            span: ctx.span,
            vis: ast::Public
        }
    }).collect();

    let methods = ast::ItemImpl(
        empty_generics(),
        None,
        @cty_to_rs(ctx, &union),
        fs
    );

    return vec!( 
        union_def,
        mk_item(ctx, "".to_owned(), methods, ast::Inherited)
    );
}

fn cenum_to_rs(ctx: &mut GenCtx, name: StrBuf, kind: IKind, items: Vec<EnumItem>) -> Vec<@ast::Item> {
    let ty = TInt(kind, Layout::zero());
    let ty_id = rust_type_id(ctx, name);
    let ty_def = ctypedef_to_rs(ctx, ty_id, &ty);
    let val_ty = cty_to_rs(ctx, &ty);
    let mut def = ty_def;

    for it in items.iter() {
        let cst = ast::ItemStatic(
            @val_ty.clone(),
            ast::MutImmutable,
            ctx.ext_cx.expr_lit(ctx.span, ast::LitIntUnsuffixed(it.val))
        );

        let id = first(rust_id(ctx, it.name.clone()));
        let val_def = @ast::Item {
                         ident: ctx.ext_cx.ident_of(id.as_slice()),
                         attrs: Vec::new(),
                         id: ast::DUMMY_NODE_ID,
                         node: cst,
                         vis: ast::Public,
                         span: ctx.span
                      };

        def.push(val_def);
    }

    return def;
}

fn mk_link_name_attr(ctx: &mut GenCtx, name: StrBuf) -> ast::Attribute {
    let lit = respan(ctx.span, ast::LitStr(
        to_intern_str(ctx, name),
        ast::CookedStr
    ));
    let attr_val = @respan(ctx.span, ast::MetaNameValue(
        to_intern_str(ctx, "link_name".to_owned()), lit
    ));
    let attr = ast::Attribute_ {
        style: ast::AttrOuter,
        value: attr_val,
        is_sugared_doc: false
    };
    respan(ctx.span, attr)
}

fn cvar_to_rs(ctx: &mut GenCtx, name: StrBuf,
                                ty: &Type,
                                is_const: bool) -> @ast::ForeignItem {
    let (rust_name, was_mangled) = rust_id(ctx, name.clone());

    let mut attrs = Vec::new();
    if was_mangled {
        attrs.push(mk_link_name_attr(ctx, name));
    }

    return @ast::ForeignItem {
              ident: ctx.ext_cx.ident_of(rust_name.as_slice()),
              attrs: attrs,
              node: ast::ForeignItemStatic(@cty_to_rs(ctx, ty), !is_const),
              id: ast::DUMMY_NODE_ID,
              span: ctx.span,
              vis: ast::Public,
           };
}

fn cfuncty_to_rs(ctx: &mut GenCtx,
                 rty: &Type,
                 aty: &[(StrBuf, Type)],
                 var: bool) -> ast::FnDecl {

    let ret = @match *rty {
        TVoid => ast::Ty {
            id: ast::DUMMY_NODE_ID,
            node: ast::TyNil,
            span: ctx.span
        },
        _ => cty_to_rs(ctx, rty)
    };

    let mut unnamed = 0;
    let args: Vec<ast::Arg> = aty.iter().map(|arg| {
        let (ref n, ref t) = *arg;

        let arg_name = if n.is_empty() {
            unnamed += 1;
            format!("arg{}", unnamed)
        } else {
            first(rust_id(ctx, n.clone()))
        };

        let arg_ty = @cty_to_rs(ctx, t);

        ast::Arg {
            ty: arg_ty,
            pat: @ast::Pat {
                 id: ast::DUMMY_NODE_ID,
                 node: ast::PatIdent(
                     ast::BindByValue(ast::MutImmutable),
                     ast::Path {
                         span: ctx.span,
                         global: false,
                         segments: Vec::from_elem(1,
                            ast::PathSegment {
                                identifier: ctx.ext_cx.ident_of(arg_name.as_slice()),
                                lifetimes: Vec::new(),
                                types: OwnedSlice::empty(),
                            }
                        )
                     },
                     None
                 ),
                 span: ctx.span
            },
            id: ast::DUMMY_NODE_ID,
        }
    }).collect();

    let var = !args.is_empty() && var;
    return ast::FnDecl {
        inputs: args,
        output: ret,
        cf: ast::Return,
        variadic: var
    };
}

fn cfunc_to_rs(ctx: &mut GenCtx, name: StrBuf, rty: &Type,
               aty: &[(StrBuf, Type)],
               var: bool) -> @ast::ForeignItem {
    let var = !aty.is_empty() && var;
    let decl = ast::ForeignItemFn(
        @cfuncty_to_rs(ctx, rty, aty, var),
        empty_generics()
    );

    let (rust_name, was_mangled) = rust_id(ctx, name.clone());

    let mut attrs = Vec::new();
    if was_mangled {
        attrs.push(mk_link_name_attr(ctx, name));
    }

    return @ast::ForeignItem {
              ident: ctx.ext_cx.ident_of(rust_name.as_slice()),
              attrs: attrs,
              node: decl,
              id: ast::DUMMY_NODE_ID,
              span: ctx.span,
              vis: ast::Public,
           };
}

fn cty_to_rs(ctx: &mut GenCtx, ty: &Type) -> ast::Ty {
    return match *ty {
        TVoid => mk_ty(ctx, true, vec!("libc".to_owned(), "c_void".to_owned())),
        TInt(i, _) => match i {
            IBool => mk_ty(ctx, true, vec!("libc".to_owned(), "c_int".to_owned())),
            ISChar => mk_ty(ctx, true, vec!("libc".to_owned(), "c_char".to_owned())),
            IUChar => mk_ty(ctx, true, vec!("libc".to_owned(), "c_uchar".to_owned())),
            IInt => mk_ty(ctx, true, vec!("libc".to_owned(), "c_int".to_owned())),
            IUInt => mk_ty(ctx, true, vec!("libc".to_owned(), "c_uint".to_owned())),
            IShort => mk_ty(ctx, true, vec!("libc".to_owned(), "c_short".to_owned())),
            IUShort => mk_ty(ctx, true, vec!("libc".to_owned(), "c_ushort".to_owned())),
            ILong => mk_ty(ctx, true, vec!("libc".to_owned(), "c_long".to_owned())),
            IULong => mk_ty(ctx, true, vec!("libc".to_owned(), "c_ulong".to_owned())),
            ILongLong => mk_ty(ctx, true, vec!("libc".to_owned(), "c_longlong".to_owned())),
            IULongLong => mk_ty(ctx, true, vec!("libc".to_owned(), "c_ulonglong".to_owned()))
        },
        TFloat(f, _) => match f {
            FFloat => mk_ty(ctx, true, vec!("libc".to_owned(), "c_float".to_owned())),
            FDouble => mk_ty(ctx, true, vec!("libc".to_owned(), "c_double".to_owned()))
        },
        TPtr(ref t, is_const, _) => {
            let id = cty_to_rs(ctx, *t);
            mk_ptrty(ctx, &id, is_const)
        },
        TArray(ref t, s, _) => {
            let ty = cty_to_rs(ctx, *t);
            mk_arrty(ctx, &ty, s)
        },
        TFunc(ref rty, ref atys, var) => {
            let decl = cfuncty_to_rs(ctx, *rty, atys.as_slice(), var);
            mk_fnty(ctx, &decl)
        },
        TNamed(ti) => {
            let id = rust_type_id(ctx, ti.borrow().name.clone());
            mk_ty(ctx, false, vec!(id))
        },
        TComp(ci) => {
            let mut c = ci.borrow_mut();
            c.name = unnamed_name(ctx, c.name.clone());
            if c.cstruct {
                mk_ty(ctx, false, vec!(struct_name(c.name.clone())))
            } else {
                mk_ty(ctx, false, vec!(union_name(c.name.clone())))
            }
        },
        TEnum(ei) => {
            let mut e = ei.borrow_mut();
            e.name = unnamed_name(ctx, e.name.clone());
            mk_ty(ctx, false, vec!(enum_name(e.name.clone())))
        }
    };
}

fn mk_ty(ctx: &mut GenCtx, global: bool, segments: Vec<StrBuf>) -> ast::Ty {
    let ty = ast::TyPath(
        ast::Path {
            span: ctx.span,
            global: global,
            segments: segments.iter().map(|s| {
                ast::PathSegment {
                    identifier: ctx.ext_cx.ident_of(s.as_slice()),
                    lifetimes: Vec::new(),
                    types: OwnedSlice::empty(),
                }
            }).collect()
        },
        option::None,
        ast::DUMMY_NODE_ID
    );

    return ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: ty,
        span: ctx.span
    };
}

fn mk_ptrty(ctx: &mut GenCtx, base: &ast::Ty, is_const: bool) -> ast::Ty {
    let ty = ast::TyPtr(ast::MutTy {
        ty: @base.clone(),
        mutbl: if is_const { ast::MutImmutable } else { ast::MutMutable }
    });

    return ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: ty,
        span: ctx.span
    };
}

fn mk_arrty(ctx: &mut GenCtx, base: &ast::Ty, n: uint) -> ast::Ty {
    let sz = ast::ExprLit(@respan(ctx.span, ast::LitUint(n as u64, ast::TyU)));
    let ty = ast::TyFixedLengthVec(
        @base.clone(),
        @ast::Expr {
            id: ast::DUMMY_NODE_ID,
            node: sz,
            span: ctx.span
        }
    );

    return ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: ty,
        span: ctx.span
    };
}

fn mk_fnty(ctx: &mut GenCtx, decl: &ast::FnDecl) -> ast::Ty {
    let fnty = ast::TyBareFn(@ast::BareFnTy {
        fn_style: ast::NormalFn,
        abi: ctx.abi,
        lifetimes: Vec::new(),
        decl: @decl.clone()
    });

    let mut segs = Vec::new();
    segs.push_all([
        ast::PathSegment {
            identifier: ctx.ext_cx.ident_of("std"),
            lifetimes: Vec::new(),
            types: OwnedSlice::empty(),
        },
        ast::PathSegment {
            identifier: ctx.ext_cx.ident_of("option"),
            lifetimes: Vec::new(),
            types: OwnedSlice::empty(),
        },
        ast::PathSegment {
            identifier: ctx.ext_cx.ident_of("Option"),
            lifetimes: Vec::new(),
            types: OwnedSlice::from_vec(Vec::from_elem(1,
                @ast::Ty {
                    id: ast::DUMMY_NODE_ID,
                    node: fnty,
                    span: ctx.span
                }
            ))
        }
    ]);

    return ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: ast::TyPath(
            ast::Path {
                span: ctx.span,
                global: true,
                segments: segs
            },
            None,
            ast::DUMMY_NODE_ID
        ),
        span: ctx.span
    };
}
