#![allow(unused_must_use)]

use std::cell::RefCell;
use std::option;
use std::io;
use std::iter;
use std::vec::Vec;
use syntax::print::pp::eof;

use syntax::abi;
use syntax::ast;
use syntax::codemap::{DUMMY_SP, dummy_spanned, ExpnInfo, NameAndSpan, MacroBang};
use syntax::ext::base;
use syntax::ext::build::AstBuilder;
use syntax::ext::expand::ExpansionConfig;
use syntax::owned_slice::OwnedSlice;
use syntax::parse;
use syntax::print::pprust;

use types::*;

struct GenCtx<'r> {
    ext_cx: base::ExtCtxt<'r>,
    unnamed_ty: uint,
    abi: abi::Abi,
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

fn to_intern_str(ctx: &mut GenCtx, s: ~str) -> parse::token::InternedString {
    let id = ctx.ext_cx.ident_of(s);
    parse::token::get_ident(id)
}

fn empty_generics() -> ast::Generics {
    ast::Generics {
        lifetimes: Vec::new(),
        ty_params: OwnedSlice::empty(),
    }
}

fn rust_id(ctx: &mut GenCtx, name: ~str) -> (~str, bool) {
    let token = parse::token::IDENT(ctx.ext_cx.ident_of(name), false);
    if parse::token::is_any_keyword(&token) || "bool" == name {
        ("_".to_owned() + name, true)
    } else {
        (name, false)
    }

}

fn rust_type_id(ctx: &mut GenCtx, name: ~str) -> ~str {
    if "bool" == name ||
        "uint" == name ||
        "u8" == name ||
        "u16" == name ||
        "u32" == name ||
        "f32" == name ||
        "f64" == name ||
        "i8" == name ||
        "i16" == name ||
        "i32" == name ||
        "i64" == name ||
        "Self" == name ||
        "str" == name {
        "_".to_owned() + name
    } else {
        let (n, _) = rust_id(ctx, name);
        n
    }
}

fn unnamed_name(ctx: &mut GenCtx, name: ~str) -> ~str {
    return if name.is_empty() {
        ctx.unnamed_ty += 1;
        format!("Unnamed{}", ctx.unnamed_ty)
    } else {
        name
    };
}

fn struct_name(name: ~str) -> ~str {
    format!("Struct_{}", name)
}

fn union_name(name: ~str) -> ~str {
    format!("Union_{}", name)
}

fn enum_name(name: ~str) -> ~str {
    format!("Enum_{}", name)
}

pub fn gen_rs(out: Box<io::Writer>, abi: ~str, links: &[~str], globs: Vec<Global>) {
    let abi = match abi.as_slice() {
        "cdecl" => abi::Cdecl,
        "stdcall" => abi::Stdcall,
        "fastcall" => abi::Fastcall,
        "aapcs" => abi::Aapcs,
        "Rust" => abi::Rust,
        "rust-intrinsic" => abi::RustIntrinsic,
        _ => abi::C
    };

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
    };
    ctx.ext_cx.bt_push(ExpnInfo {
        call_site: DUMMY_SP,
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
            _ => { fail!("generate global variables".to_owned()) }
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
                    _ => { fail!("generate functions".to_owned()) }
                }
            },
            _ => { fail!("generate functions".to_owned()) }
        }
    }).collect();

    defs.push(mk_extern(&mut ctx, links, vars, funcs));

    let views = Vec::from_elem(1, mk_import(&mut ctx, &["libc".to_owned()]));

    let crate_ = ast::Crate {
        module: ast::Mod {
            inner: DUMMY_SP,
            view_items: views,
            items: defs,
        },
        attrs: Vec::new(),
        config: Vec::new(),
        span: DUMMY_SP
    };

    let mut ps = pprust::rust_printer(out);
    ps.s.out.write("/* automatically generated by rust-bindgen */\n\n".as_bytes());

    ps.print_mod(&crate_.module, crate_.attrs.as_slice());
    ps.print_remaining_comments();
    eof(&mut ps.s);

    ps.s.out.flush();
}

fn mk_import(ctx: &mut GenCtx, path: &[~str]) -> ast::ViewItem {
    let view = ast::ViewItemUse(
        @dummy_spanned(
            ast::ViewPathGlob(
                ast::Path {
                    span: DUMMY_SP,
                    global: false,
                    segments: path.iter().map(|p|
                        ast::PathSegment {
                            identifier: ctx.ext_cx.ident_of((*p).clone()),
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
              span: DUMMY_SP
           };
}

fn mk_extern(ctx: &mut GenCtx, links: &[~str],
             vars: Vec<@ast::ForeignItem>,
             funcs: Vec<@ast::ForeignItem>) -> @ast::Item {
    let attrs = if links.is_empty() {
        Vec::new()
    } else {
        links.iter().map(|ref l| {
            let link_name = @dummy_spanned(ast::MetaNameValue(
                to_intern_str(ctx, "name".to_owned()),
                dummy_spanned(ast::LitStr(
                    to_intern_str(ctx, l.to_owned()),
                    ast::CookedStr
                ))
            ));
            let link_args = dummy_spanned(ast::Attribute_ {
                style: ast::AttrOuter,
                value: @dummy_spanned(ast::MetaList(
                    to_intern_str(ctx, "link".to_owned()),
                    Vec::from_elem(1, link_name))
                ),
                is_sugared_doc: false
            });
            link_args
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
              vis: ast::Public,
              span: DUMMY_SP
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
              check(a.name, b.name)
          },
          (&GComp(ci1), &GComp(ci2)) => {
              let a = ci1.borrow();
              let b = ci2.borrow();
              check(a.name, b.name)
          },
          (&GCompDecl(ci1), &GCompDecl(ci2)) => {
              let a = ci1.borrow();
              let b = ci2.borrow();
              check(a.name, b.name)
          },
          (&GEnum(ei1), &GEnum(ei2)) => {
              let a = ei1.borrow();
              let b = ei2.borrow();
              check(a.name, b.name)
          },
          (&GEnumDecl(ei1), &GEnumDecl(ei2)) => {
              let a = ei1.borrow();
              let b = ei2.borrow();
              check(a.name, b.name)
          },
          (&GVar(vi1), &GVar(vi2)) => {
              let a = vi1.borrow();
              let b = vi2.borrow();
              check(a.name, b.name)
          },
          (&GFunc(vi1), &GFunc(vi2)) => {
              let a = vi1.borrow();
              let b = vi2.borrow();
              check(a.name, b.name)
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

fn ctypedef_to_rs(ctx: &mut GenCtx, name: ~str, ty: &Type) -> Vec<@ast::Item> {
    fn mk_item(ctx: &mut GenCtx, name: ~str, ty: &Type) -> @ast::Item {
        let rust_name = rust_type_id(ctx, name);
        let rust_ty = cty_to_rs(ctx, ty);
        let base = ast::ItemTy(
            @ast::Ty {
                id: ast::DUMMY_NODE_ID,
                node: rust_ty.node,
                span: DUMMY_SP,
            },
            empty_generics()
        );

        return @ast::Item {
                  ident: ctx.ext_cx.ident_of(rust_name),
                  attrs: Vec::new(),
                  id: ast::DUMMY_NODE_ID,
                  node: base,
                  vis: ast::Public,
                  span: DUMMY_SP
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

fn cstruct_to_rs(ctx: &mut GenCtx, name: ~str, fields: Vec<FieldInfo>) -> @ast::Item {
    let mut unnamed = 0;
    let fs = fields.iter().map(|f| {
        let f_name = if f.name.is_empty() || "_" == f.name {
            unnamed += 1;
            format!("unnamed_field{}", unnamed)
        } else {
            rust_type_id(ctx, f.name.clone())
        };

        let f_ty = @cty_to_rs(ctx, &f.ty);

        dummy_spanned(ast::StructField_ {
            kind: ast::NamedField(
                ctx.ext_cx.ident_of(f_name),
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
    return @ast::Item { ident: ctx.ext_cx.ident_of(id),
              attrs: Vec::new(),
              id: ast::DUMMY_NODE_ID,
              node: def,
              vis: ast::Public,
              span: DUMMY_SP
           };
}

fn cunion_to_rs(ctx: &mut GenCtx, name: ~str, layout: Layout, fields: Vec<FieldInfo>) -> Vec<@ast::Item> {
    fn mk_item(ctx: &mut GenCtx, name: ~str, item: ast::Item_, vis: ast::Visibility) -> @ast::Item {
        return @ast::Item {
                  ident: ctx.ext_cx.ident_of(name),
                  attrs: Vec::new(),
                  id: ast::DUMMY_NODE_ID,
                  node: item,
                  vis: vis,
                  span: DUMMY_SP
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
    let base_ty = mk_ty(ctx, ty_name.to_owned());
    let data_ty = @mk_arrty(ctx, &base_ty, data_len);
    let data = dummy_spanned(ast::StructField_ {
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
        let f_name = if f.name.is_empty() || "_" == f.name {
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
            span: DUMMY_SP
        };

        @ast::Method {
            ident: ctx.ext_cx.ident_of(f_name),
            attrs: Vec::new(),
            generics: empty_generics(),
            explicit_self: dummy_spanned(ast::SelfRegion(None, ast::MutMutable)),
            fn_style: ast::NormalFn,
            decl: @ast::FnDecl {
                inputs: Vec::from_elem(1, ast::Arg::new_self(DUMMY_SP, ast::MutImmutable)),
                output: ret_ty,
                cf: ast::Return,
                variadic: false
            },
            body: body,
            id: ast::DUMMY_NODE_ID,
            span: DUMMY_SP,
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

fn cenum_to_rs(ctx: &mut GenCtx, name: ~str, kind: IKind, items: Vec<EnumItem>) -> Vec<@ast::Item> {
    let ty = TInt(kind, Layout::zero());
    let ty_id = rust_type_id(ctx, name);
    let ty_def = ctypedef_to_rs(ctx, ty_id, &ty);
    let val_ty = cty_to_rs(ctx, &ty);
    let mut def = ty_def;

    for it in items.iter() {
        let cst = ast::ItemStatic(
            @val_ty.clone(),
            ast::MutImmutable,
            ctx.ext_cx.expr_int(DUMMY_SP, it.val)
        );

        let id = first(rust_id(ctx, it.name.clone()));
        let val_def = @ast::Item {
                         ident: ctx.ext_cx.ident_of(id),
                         attrs: Vec::new(),
                         id: ast::DUMMY_NODE_ID,
                         node: cst,
                         vis: ast::Public,
                         span: DUMMY_SP
                      };

        def.push(val_def);
    }

    return def;
}

fn mk_link_name_attr(ctx: &mut GenCtx, name: ~str) -> ast::Attribute {
    let lit = dummy_spanned(ast::LitStr(
        to_intern_str(ctx, name),
        ast::CookedStr
    ));
    let attr_val = @dummy_spanned(ast::MetaNameValue(
        to_intern_str(ctx, "link_name".to_owned()), lit
    ));
    let attr = ast::Attribute_ {
        style: ast::AttrOuter,
        value: attr_val,
        is_sugared_doc: false
    };
    dummy_spanned(attr)
}

fn cvar_to_rs(ctx: &mut GenCtx, name: ~str,
                                ty: &Type,
                                is_const: bool) -> @ast::ForeignItem {
    let (rust_name, was_mangled) = rust_id(ctx, name.clone());

    let mut attrs = Vec::new();
    if was_mangled {
        attrs.push(mk_link_name_attr(ctx, name));
    }

    return @ast::ForeignItem {
              ident: ctx.ext_cx.ident_of(rust_name),
              attrs: attrs,
              node: ast::ForeignItemStatic(@cty_to_rs(ctx, ty), !is_const),
              id: ast::DUMMY_NODE_ID,
              span: DUMMY_SP,
              vis: ast::Public,
           };
}

fn cfuncty_to_rs(ctx: &mut GenCtx,
                 rty: &Type,
                 aty: &[(~str, Type)],
                 var: bool) -> ast::FnDecl {

    let ret = @match *rty {
        TVoid => ast::Ty {
            id: ast::DUMMY_NODE_ID,
            node: ast::TyNil,
            span: DUMMY_SP
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
                         span: DUMMY_SP,
                         global: false,
                         segments: Vec::from_elem(1,
                            ast::PathSegment {
                                identifier: ctx.ext_cx.ident_of(arg_name),
                                lifetimes: Vec::new(),
                                types: OwnedSlice::empty(),
                            }
                        )
                     },
                     None
                 ),
                 span: DUMMY_SP
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

fn cfunc_to_rs(ctx: &mut GenCtx, name: ~str, rty: &Type,
               aty: &[(~str, Type)],
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
              ident: ctx.ext_cx.ident_of(rust_name),
              attrs: attrs,
              node: decl,
              id: ast::DUMMY_NODE_ID,
              span: DUMMY_SP,
              vis: ast::Public,
           };
}

fn cty_to_rs(ctx: &mut GenCtx, ty: &Type) -> ast::Ty {
    return match *ty {
        TVoid => mk_ty(ctx, "c_void".to_owned()),
        TInt(i, _) => match i {
            IBool => mk_ty(ctx, "c_int".to_owned()),
            ISChar => mk_ty(ctx, "c_char".to_owned()),
            IUChar => mk_ty(ctx, "c_uchar".to_owned()),
            IInt => mk_ty(ctx, "c_int".to_owned()),
            IUInt => mk_ty(ctx, "c_uint".to_owned()),
            IShort => mk_ty(ctx, "c_short".to_owned()),
            IUShort => mk_ty(ctx, "c_ushort".to_owned()),
            ILong => mk_ty(ctx, "c_long".to_owned()),
            IULong => mk_ty(ctx, "c_ulong".to_owned()),
            ILongLong => mk_ty(ctx, "c_longlong".to_owned()),
            IULongLong => mk_ty(ctx, "c_ulonglong".to_owned())
        },
        TFloat(f, _) => match f {
            FFloat => mk_ty(ctx, "c_float".to_owned()),
            FDouble => mk_ty(ctx, "c_double".to_owned())
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
            mk_ty(ctx, id)
        },
        TComp(ci) => {
            let mut c = ci.borrow_mut();
            c.name = unnamed_name(ctx, c.name.clone());
            if c.cstruct {
                mk_ty(ctx, struct_name(c.name.clone()))
            } else {
                mk_ty(ctx, union_name(c.name.clone()))
            }
        },
        TEnum(ei) => {
            let mut e = ei.borrow_mut();
            e.name = unnamed_name(ctx, e.name.clone());
            mk_ty(ctx, enum_name(e.name.clone()))
        }
    };
}

fn mk_ty(ctx: &mut GenCtx, name: ~str) -> ast::Ty {
    let ty = ast::TyPath(
        ast::Path {
            span: DUMMY_SP,
            global: false,
            segments: Vec::from_elem(1,
                ast::PathSegment {
                    identifier: ctx.ext_cx.ident_of(name),
                    lifetimes: Vec::new(),
                    types: OwnedSlice::empty(),
                }
            )
        },
        option::None,
        ast::DUMMY_NODE_ID
    );

    return ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: ty,
        span: DUMMY_SP
    };
}

fn mk_ptrty(_ctx: &mut GenCtx, base: &ast::Ty, is_const: bool) -> ast::Ty {
    let ty = ast::TyPtr(ast::MutTy {
        ty: @base.clone(),
        mutbl: if is_const { ast::MutImmutable } else { ast::MutMutable }
    });

    return ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: ty,
        span: DUMMY_SP
    };
}

fn mk_arrty(_ctx: &mut GenCtx, base: &ast::Ty, n: uint) -> ast::Ty {
    let sz = ast::ExprLit(@dummy_spanned(ast::LitUint(n as u64, ast::TyU)));
    let ty = ast::TyFixedLengthVec(
        @base.clone(),
        @ast::Expr {
            id: ast::DUMMY_NODE_ID,
            node: sz,
            span: DUMMY_SP
        }
    );

    return ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: ty,
        span: DUMMY_SP
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
                    span: DUMMY_SP
                }
            ))
        }
    ]);

    return ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: ast::TyPath(
            ast::Path {
                span: DUMMY_SP,
                global: true,
                segments: segs
            },
            None,
            ast::DUMMY_NODE_ID
        ),
        span: DUMMY_SP
    };
}
