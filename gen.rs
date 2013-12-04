use std::borrow;
use std::option;
use std::io;
use std::iter;

use syntax::abi;
use syntax::ast;
use syntax::codemap::{dummy_sp, dummy_spanned, ExpnInfo, NameAndSpan};
use syntax::ast_util::*;
use syntax::ext::base;
use syntax::ext::build::AstBuilder;
use syntax::parse;
use syntax::print::pprust;
use syntax::opt_vec;

use types::*;

struct GenCtx {
    ext_cx: @base::ExtCtxt,
    unnamed_ty: uint,
    abis: abi::AbiSet
}

fn empty_generics() -> ast::Generics {
    ast::Generics {
        lifetimes: opt_vec::Empty,
        ty_params: opt_vec::Empty
    }
}

fn rust_id(ctx: &mut GenCtx, name: ~str) -> (~str, bool) {
    let token = parse::token::IDENT(ctx.ext_cx.ident_of(name), false);
    if parse::token::is_any_keyword(&token) || "bool" == name {
        (~"_" + name, true)
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
        ~"_" + name
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

pub fn gen_rs(out: @mut io::Writer, abi: ~str, link: &Option<~str>, globs: ~[Global]) {
    let abis = match abi {
        ~"cdecl" => abi::AbiSet::from(abi::Cdecl),
        ~"stdcall" => abi::AbiSet::from(abi::Stdcall),
        ~"fastcall" => abi::AbiSet::from(abi::Fastcall),
        ~"aapcs" => abi::AbiSet::from(abi::Aapcs),
        ~"Rust" => abi::AbiSet::Rust(),
        ~"rust-intrinsic" => abi::AbiSet::Intrinsic(),
        _ => abi::AbiSet::C()
    };

    let mut ctx = GenCtx { ext_cx: base::ExtCtxt::new(parse::new_parse_sess(None), ~[]),
                           unnamed_ty: 0,
                           abis: abis
                         };
    ctx.ext_cx.bt_push(ExpnInfo {
        call_site: dummy_sp(),
        callee: NameAndSpan { name: @"", span: None }
    });
    let uniq_globs = tag_dup_decl(globs);

    let mut fs = ~[];
    let mut vs = ~[];
    let mut gs = ~[];
    for g in uniq_globs.move_iter() {
        match g {
            GOther => {}
            GFunc(_) => fs.push(g),
            GVar(_) => vs.push(g),
            _ => gs.push(g)
        }
    }

    let mut defs = ~[];
    gs = remove_redundant_decl(gs);

    for g in gs.move_iter() {
        match g {
            GType(ti) => defs.push_all(ctypedef_to_rs(&mut ctx, ti.name.clone(), &ti.ty)),
            GCompDecl(ci) => {
                ci.name = unnamed_name(&mut ctx, ci.name.clone());
                if ci.cstruct {
                    defs.push_all(ctypedef_to_rs(&mut ctx, struct_name(ci.name.clone()), &TVoid))
                } else {
                    defs.push_all(ctypedef_to_rs(&mut ctx, union_name(ci.name.clone()), &TVoid))
                }
            },
            GComp(ci) => {
                ci.name = unnamed_name(&mut ctx, ci.name.clone());
                if ci.cstruct {
                    defs.push(cstruct_to_rs(&mut ctx, struct_name(ci.name.clone()),
                                            // this clone is necessary to prevent dynamic borrow
                                            // check errors.
                                            // FIXME: remove the @mut in types.rs to fix this
                                            ci.fields.clone()))
                } else {
                    defs.push_all(cunion_to_rs(&mut ctx, union_name(ci.name.clone()),
                                               ci.fields, ci.layout))
                }
            },
            GEnumDecl(ei) => {
                ei.name = unnamed_name(&mut ctx, ei.name.clone());
                defs.push_all(ctypedef_to_rs(&mut ctx, enum_name(ei.name.clone()), &TVoid))
            },
            GEnum(ei) => {
                ei.name = unnamed_name(&mut ctx, ei.name.clone());
                defs.push_all(cenum_to_rs(&mut ctx, enum_name(ei.name.clone()), ei.items, ei.kind))
            },
            _ => { }
        }
    }

    let vars = vs.move_iter().map(|v| {
        match v {
            GVar(vi) => cvar_to_rs(&mut ctx, vi.name.clone(), &vi.ty, vi.is_const),
            _ => { fail!(~"generate global variables") }
        }
    }).collect();

    let funcs = fs.move_iter().map(|f| {
        match f {
            GFunc(vi) => {
                match vi.ty {
                    TFunc(ref rty, ref aty, var) => cfunc_to_rs(&mut ctx, vi.name.clone(),
                                                                *rty, *aty, var),
                    _ => { fail!(~"generate functions") }
                }
            },
            _ => { fail!(~"generate functions") }
        }
    }).collect();

    let views = ~[mk_import(&mut ctx, &[~"std", ~"libc"])];
    defs.push(mk_extern(&mut ctx, link, vars, funcs));

    let crate = ast::Crate {
        module: ast::_mod {
            view_items: views,
            items: defs,
        },
        attrs: ~[],
        config: ~[],
        span: dummy_sp()
    };

    let ps = pprust::rust_printer(out, parse::token::get_ident_interner());
    out.write("/* automatically generated by rust-bindgen */\n\n".as_bytes());
    pprust::print_crate_(ps, &crate);
}

fn mk_import(ctx: &mut GenCtx, path: &[~str]) -> ast::view_item {
    let view = ast::view_item_use(~[
        @dummy_spanned(
            ast::view_path_glob(
                ast::Path {
                    span: dummy_sp(),
                    global: false,
                    segments: path.map(|p|
                        ast::PathSegment {
                            identifier: ctx.ext_cx.ident_of((*p).clone()),
                            lifetimes: opt_vec::Empty,
                            types: opt_vec::Empty
                        }
                    )
                },
                ast::DUMMY_NODE_ID
            )
        )
    ]);

    return ast::view_item {
              node: view,
              attrs: ~[],
              vis: ast::inherited,
              span: dummy_sp()
           };
}

fn mk_extern(ctx: &mut GenCtx, link: &Option<~str>,
                           vars: ~[@ast::foreign_item],
                           funcs: ~[@ast::foreign_item]) -> @ast::item {
    let attrs;
    match *link {
        None => attrs = ~[],
        Some(ref l) => {
            let link_name = @dummy_spanned(ast::MetaNameValue(
                @"name", dummy_spanned(ast::lit_str(l.to_managed(), ast::CookedStr))
            ));
            let link_args = dummy_spanned(ast::Attribute_ {
                style: ast::AttrOuter,
                value: @dummy_spanned(ast::MetaList(@"link", ~[link_name])),
                is_sugared_doc: false
            });
            attrs = ~[link_args];
        }
    }

    let ext = ast::item_foreign_mod(ast::foreign_mod {
        abis: ctx.abis,
        view_items: ~[],
        items: vars + funcs
    });

    return @ast::item {
              ident: ctx.ext_cx.ident_of(""),
              attrs: attrs,
              id: ast::DUMMY_NODE_ID,
              node: ext,
              vis: ast::public,
              span: dummy_sp()
           };
}

fn remove_redundant_decl(gs: ~[Global]) -> ~[Global] {
    fn check_decl(a: &Global, ty: &Type) -> bool {
        match *a {
          GComp(ci1) => match *ty {
              TComp(ci2) => {
                  borrow::ref_eq(ci1, ci2) && ci1.name.is_empty()
              },
              _ => false
          },
          GEnum(ei1) => match *ty {
              TEnum(ei2) => {
                  borrow::ref_eq(ei1, ei2) && ei1.name.is_empty()
              },
              _ => false
          },
          _ => false
        }
    }

    let typedefs: ~[Type] = gs.iter().filter_map(|g|
        match *g {
            GType(ref ti) => Some(ti.ty.clone()),
            _ => None
        }
    ).collect();

    return gs.move_iter().filter(|g|
        !typedefs.iter().any(|t| check_decl(g, t))
    ).collect();
}

fn tag_dup_decl(gs: ~[Global]) -> ~[Global] {
    fn check(name1: &str, name2: &str) -> bool {
        !name1.is_empty() && name1 == name2
    }

    fn check_dup(g1: &Global, g2: &Global) -> bool {
        match (g1, g2) {
          (&GType(ti1), &GType(ti2)) => check(ti1.name, ti2.name),
          (&GComp(ci1), &GComp(ci2)) => check(ci1.name, ci2.name),
          (&GCompDecl(ci1), &GCompDecl(ci2)) => check(ci1.name, ci2.name),
          (&GEnum(ei1), &GEnum(ei2)) => check(ei1.name, ei2.name),
          (&GEnumDecl(ei1), &GEnumDecl(ei2)) => check(ei1.name, ei2.name),
          (&GVar(vi1), &GVar(vi2)) => check(vi1.name, vi2.name),
          (&GFunc(vi1), &GFunc(vi2)) => check(vi1.name, vi2.name),
          _ => false
        }
    }

    let len = gs.len();
    let mut res = ~[];
    res.push(gs[0]);

    for i in iter::range(1, len) {
        let mut dup = false;
        for j in iter::range(0, i-1) {
            if check_dup(&gs[i], &gs[j]) {
                dup = true;
                break;
            }
        }
        if !dup {
            res.push(gs[i]);
        }
    }

    return res;
}

fn ctypedef_to_rs(ctx: &mut GenCtx, name: ~str, ty: &Type) -> ~[@ast::item] {
    fn mk_item(ctx: &mut GenCtx, name: ~str, ty: &Type) -> @ast::item {
        let rust_name = rust_type_id(ctx, name);
        let rust_ty = cty_to_rs(ctx, ty);
        let base = ast::item_ty(
            @ast::Ty {
                id: ast::DUMMY_NODE_ID,
                node: rust_ty.node,
                span: dummy_sp(),
            },
            empty_generics()
        );

        return @ast::item {
                  ident: ctx.ext_cx.ident_of(rust_name),
                  attrs: ~[],
                  id: ast::DUMMY_NODE_ID,
                  node: base,
                  vis: ast::public,
                  span: dummy_sp()
               };
    }

    return match *ty {
        TComp(ci) => {
            if ci.name.is_empty() {
                ci.name = name.clone();
                if ci.cstruct {
                    ~[cstruct_to_rs(ctx, name, ci.fields)]
                } else {
                    cunion_to_rs(ctx, name, ci.fields, ci.layout)
                }
            } else {
                ~[mk_item(ctx, name, ty)]
            }
        },
        TEnum(ei) => {
            if ei.name.is_empty() {
                ei.name = name.clone();
                cenum_to_rs(ctx, name, ei.items, ei.kind)
            } else {
                ~[mk_item(ctx, name, ty)]
            }
        },
        _ => ~[mk_item(ctx, name, ty)]
    }
}

fn cstruct_to_rs(ctx: &mut GenCtx, name: ~str, fields: &[FieldInfo]) -> @ast::item {
    let mut unnamed = 0;
    let fs = fields.map(|f| {
        let f_name = if f.name.is_empty() {
            unnamed += 1;
            format!("unnamed_field{}", unnamed)
        } else {
            rust_type_id(ctx, f.name.clone())
        };

        let f_ty = @cty_to_rs(ctx, &f.ty);

        dummy_spanned(ast::struct_field_ {
            kind: ast::named_field(
                ctx.ext_cx.ident_of(f_name),
                ast::inherited
            ),
            id: ast::DUMMY_NODE_ID,
            ty: f_ty,
            attrs: ~[]
        })
    });

    let def = ast::item_struct(
        @ast::struct_def {
           fields: fs,
           ctor_id: None
        },
        empty_generics()
    );

    let id = rust_type_id(ctx, name);
    return @ast::item { ident: ctx.ext_cx.ident_of(id),
              attrs: ~[],
              id: ast::DUMMY_NODE_ID,
              node: def,
              vis: ast::public,
              span: dummy_sp()
           };
}

fn cunion_to_rs(ctx: &mut GenCtx, name: ~str, fields: &[FieldInfo], layout: Layout) -> ~[@ast::item] {
    fn mk_item(ctx: &mut GenCtx, name: ~str, item: ast::item_, vis: ast::visibility) -> @ast::item {
        return @ast::item {
                  ident: ctx.ext_cx.ident_of(name),
                  attrs: ~[],
                  id: ast::DUMMY_NODE_ID,
                  node: item,
                  vis: vis,
                  span: dummy_sp()
               };
    }

    let ext_cx = ctx.ext_cx;
    let ci = @mut CompInfo::new(name.clone(), false, fields.to_owned(), layout);
    let union = TNamed(@mut TypeInfo::new(name.clone(), TComp(ci)));

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
    let data = dummy_spanned(ast::struct_field_ {
        kind: ast::named_field(
            ctx.ext_cx.ident_of("data"),
            ast::inherited
        ),
        id: ast::DUMMY_NODE_ID,
        ty: data_ty,
        attrs: ~[]
    });

    let def = ast::item_struct(
        @ast::struct_def {
           fields: ~[data],
           ctor_id: None
        },
        empty_generics()
    );
    let union_id = rust_type_id(ctx, name);
    let union_def = mk_item(ctx, union_id, def, ast::public);

    let expr = quote_expr!(
        ext_cx,
        unsafe { ::std::cast::transmute(::std::ptr::to_mut_unsafe_ptr(self)) }
    );
    let mut unnamed = 0;
    let fs = fields.map(|f| {
        let f_name = if f.name.is_empty() {
            unnamed += 1;
            format!("unnamed_field{}", unnamed)
        } else {
            rust_id(ctx, f.name.clone()).first()
        };

        let ret_ty = @cty_to_rs(ctx, &TPtr(~f.ty.clone(), false, Layout::zero()));
        let body = @ast::Block {
            view_items: ~[],
            stmts: ~[],
            expr: Some(expr),
            id: ast::DUMMY_NODE_ID,
            rules: ast::DefaultBlock,
            span: dummy_sp()
        };

        @ast::method {
            ident: ext_cx.ident_of(f_name),
            attrs: ~[],
            generics: empty_generics(),
            explicit_self: dummy_spanned(ast::sty_region(None, ast::MutMutable)),
            purity: ast::impure_fn,
            decl: @ast::fn_decl {
                inputs: ~[],
                output: ret_ty,
                cf: ast::return_val,
                variadic: false
            },
            body: body,
            id: ast::DUMMY_NODE_ID,
            span: dummy_sp(),
            self_id: union_def.id,
            vis: ast::public
        }
    });

    let methods = ast::item_impl(
        empty_generics(),
        None,
        @cty_to_rs(ctx, &union),
        fs
    );

    return ~[
        union_def,
        mk_item(ctx, ~"", methods, ast::inherited)
    ];
}

fn cenum_to_rs(ctx: &mut GenCtx, name: ~str, items: &[EnumItem], kind: IKind) -> ~[@ast::item] {
    let ty = TInt(kind, Layout::zero());
    let ty_id = rust_type_id(ctx, name);
    let ty_def = ctypedef_to_rs(ctx, ty_id, &ty);
    let val_ty = cty_to_rs(ctx, &ty);
    let mut def = ty_def;

    for it in items.iter() {
        let cst = ast::item_static(
            @val_ty.clone(),
            ast::MutImmutable,
            ctx.ext_cx.expr_int(dummy_sp(), it.val)
        );

        let id = rust_id(ctx, it.name.clone()).first();
        let val_def = @ast::item {
                         ident: ctx.ext_cx.ident_of(id),
                         attrs: ~[],
                         id: ast::DUMMY_NODE_ID,
                         node: cst,
                         vis: ast::public,
                         span: dummy_sp()
                      };

        def.push(val_def);
    }

    return def;
}

fn mk_link_name_attr(name: ~str) -> ast::Attribute {
    let lit = dummy_spanned(ast::lit_str(name.to_managed(), ast::CookedStr));
    let attr_val = @dummy_spanned(ast::MetaNameValue(@"link_name", lit));
    let attr = ast::Attribute_ {
        style: ast::AttrOuter,
        value: attr_val,
        is_sugared_doc: false
    };
    dummy_spanned(attr)
}

fn cvar_to_rs(ctx: &mut GenCtx, name: ~str,
                                ty: &Type,
                                is_const: bool) -> @ast::foreign_item {
    let (rust_name, was_mangled) = rust_id(ctx, name.clone());

    let mut attrs = ~[];
    if was_mangled {
        attrs.push(mk_link_name_attr(name));
    }

    return @ast::foreign_item {
              ident: ctx.ext_cx.ident_of(rust_name),
              attrs: attrs,
              node: ast::foreign_item_static(@cty_to_rs(ctx, ty), !is_const),
              id: ast::DUMMY_NODE_ID,
              span: dummy_sp(),
              vis: ast::public,
           };
}

fn cfuncty_to_rs(ctx: &mut GenCtx,
                 rty: &Type,
                 aty: &[(~str, Type)],
                 var: bool) -> ast::fn_decl {

    let ret = @match *rty {
        TVoid => ast::Ty {
            id: ast::DUMMY_NODE_ID,
            node: ast::ty_nil,
            span: dummy_sp()
        },
        _ => cty_to_rs(ctx, rty)
    };

    let mut unnamed = 0;
    let args = aty.iter().map(|arg| {
        let (ref n, ref t) = *arg;

        let arg_name = if n.is_empty() {
            unnamed += 1;
            format!("arg{}", unnamed)
        } else {
            rust_id(ctx, n.clone()).first()
        };

        let arg_ty = @cty_to_rs(ctx, t);

        ast::arg {
            ty: arg_ty,
            pat: @ast::Pat {
                 id: ast::DUMMY_NODE_ID,
                 node: ast::PatIdent(
                     ast::BindByValue(ast::MutImmutable),
                     ast::Path {
                         span: dummy_sp(),
                         global: false,
                         segments: ~[
                            ast::PathSegment {
                                identifier: ctx.ext_cx.ident_of(arg_name),
                                lifetimes: opt_vec::Empty,
                                types: opt_vec::Empty
                            }
                        ]
                     },
                     None
                 ),
                 span: dummy_sp()
            },
            id: ast::DUMMY_NODE_ID,
        }
    }).collect();

    return ast::fn_decl {
        inputs: args,
        output: ret,
        cf: ast::return_val,
        variadic: var
    };
}

fn cfunc_to_rs(ctx: &mut GenCtx, name: ~str, rty: &Type,
               aty: &[(~str, Type)],
               var: bool) -> @ast::foreign_item {
    let decl = ast::foreign_item_fn(
        @cfuncty_to_rs(ctx, rty, aty, var),
        empty_generics()
    );

    let (rust_name, was_mangled) = rust_id(ctx, name.clone());

    let mut attrs = ~[];
    if was_mangled {
        attrs.push(mk_link_name_attr(name));
    }

    return @ast::foreign_item {
              ident: ctx.ext_cx.ident_of(rust_name),
              attrs: attrs,
              node: decl,
              id: ast::DUMMY_NODE_ID,
              span: dummy_sp(),
              vis: ast::public,
           };
}

fn cty_to_rs(ctx: &mut GenCtx, ty: &Type) -> ast::Ty {
    return match *ty {
        TVoid => mk_ty(ctx, ~"c_void"),
        TInt(i, _) => match i {
            IBool => mk_ty(ctx, ~"c_int"),
            ISChar => mk_ty(ctx, ~"c_schar"),
            IUChar => mk_ty(ctx, ~"c_uchar"),
            IInt => mk_ty(ctx, ~"c_int"),
            IUInt => mk_ty(ctx, ~"c_uint"),
            IShort => mk_ty(ctx, ~"c_short"),
            IUShort => mk_ty(ctx, ~"c_ushort"),
            ILong => mk_ty(ctx, ~"c_long"),
            IULong => mk_ty(ctx, ~"c_ulong"),
            ILongLong => mk_ty(ctx, ~"c_longlong"),
            IULongLong => mk_ty(ctx, ~"c_ulonglong")
        },
        TFloat(f, _) => match f {
            FFloat => mk_ty(ctx, ~"c_float"),
            FDouble => mk_ty(ctx, ~"c_double")
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
            let decl = cfuncty_to_rs(ctx, *rty, *atys, var);
            mk_fnty(ctx, &decl)
        },
        TNamed(ti) => {
            let id = rust_type_id(ctx, ti.name.clone());
            mk_ty(ctx, id)
        },
        TComp(ci) => {
            ci.name = unnamed_name(ctx, ci.name.clone());
            if ci.cstruct {
                mk_ty(ctx, struct_name(ci.name.clone()))
            } else {
                mk_ty(ctx, union_name(ci.name.clone()))
            }
        },
        TEnum(ei) => {
            ei.name = unnamed_name(ctx, ei.name.clone());
            mk_ty(ctx, enum_name(ei.name.clone()))
        }
    };
}

fn mk_ty(ctx: &mut GenCtx, name: ~str) -> ast::Ty {
    let ty = ast::ty_path(
        ast::Path {
            span: dummy_sp(),
            global: false,
            segments: ~[
                ast::PathSegment {
                    identifier: ctx.ext_cx.ident_of(name),
                    lifetimes: opt_vec::Empty,
                    types: opt_vec::Empty
                }
            ]
        },
        option::None,
        ast::DUMMY_NODE_ID
    );

    return ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: ty,
        span: dummy_sp()
    };
}

fn mk_ptrty(_ctx: &mut GenCtx, base: &ast::Ty, is_const: bool) -> ast::Ty {
    let ty = ast::ty_ptr(ast::mt{
        ty: @base.clone(),
        mutbl: if is_const { ast::MutImmutable } else { ast::MutMutable }
    });

    return ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: ty,
        span: dummy_sp()
    };
}

fn mk_arrty(_ctx: &mut GenCtx, base: &ast::Ty, n: uint) -> ast::Ty {
    let sz = ast::ExprLit(@dummy_spanned(ast::lit_uint(n as u64, ast::ty_u)));
    let ty = ast::ty_fixed_length_vec(
        ast::mt {
            ty: @base.clone(),
            mutbl: ast::MutImmutable
        },
        @ast::Expr {
            id: ast::DUMMY_NODE_ID,
            node: sz,
            span: dummy_sp()
        }
    );

    return ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: ty,
        span: dummy_sp()
    };
}

fn mk_fnty(ctx: &mut GenCtx, decl: &ast::fn_decl) -> ast::Ty {
    let fnty = ast::ty_bare_fn(@ast::TyBareFn {
        purity: ast::impure_fn,
        abis: ctx.abis,
        lifetimes: opt_vec::Empty,
        decl: @decl.clone()
    });

    return ast::Ty {
        id: ast::DUMMY_NODE_ID,
        node: fnty,
        span: dummy_sp()
    };
}
