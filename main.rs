#![allow(non_uppercase_pattern_statics)]
#![allow(unused_must_use)]

use std::cell::RefCell;
use collections::{HashMap, HashSet};
use std::{os, path};
use std::io;
use std::io::fs;

use il = types;
use types::*;
use cx = clang;
use clang::*;
use clang::ll::*;
use gen::*;

struct BindGenCtx {
    match_pat: Vec<~str>,
    abi: ~str,
    builtins: bool,
    links: Vec<~str>,
    name: HashMap<Cursor, Global>,
    globals: Vec<Global>,
    builtin_defs: Vec<Cursor>,
    builtin_names: HashSet<~str>,
    emit_ast: bool,
    fail_on_bitfield: bool
}

enum ParseResult {
    CmdUsage,
    ParseOk(Vec<~str>, Box<BindGenCtx>, Box<io::Writer>),
    ParseErr(~str)
}

fn parse_args(args: &[~str]) -> ParseResult {
    let mut clang_args = vec!();
    let args_len = args.len();

    let mut out = box io::BufferedWriter::new(io::stdout()) as Box<io::Writer>;
    let mut pat = vec!();
    let mut links = vec!();
    let mut abi = "C".to_owned();
    let mut builtins = false;
    let mut emit_ast = false;
    let mut fail_on_bitfield = true;

    if args_len == 0u {
        return CmdUsage;
    }

    let mut ix = 0u;
    while ix < args_len {
        if args[ix].len() > 2 && args[ix].slice_to(2) == "-l" {
            links.push(args[ix].slice_from(2).to_owned());
            ix += 1u;
        } else {
            match args[ix].as_slice() {
                "--help" | "-h" => {
                    return CmdUsage;
                }
                "-emit-clang-ast" => {
                  emit_ast = true;
                  ix += 1u;
                }
                "-o" => {
                    if ix + 1u >= args_len {
                        return ParseErr("Missing output filename".to_owned());
                    }
                    let path = path::Path::new(args[ix + 1].clone());
                    match fs::File::create(&path) {
                      Ok(f) => { out = box io::BufferedWriter::new(f) as Box<io::Writer>; }
                      Err(_) => { return ParseErr(format!("Open {} failed", args[ix + 1])); }
                    }
                    ix += 2u;
                }
                "-l" => {
                    if ix + 1u >= args_len {
                        return ParseErr("Missing link name".to_owned());
                    }
                    links.push(args[ix + 1u].clone());
                    ix += 2u;
                }
                "-match" => {
                    if ix + 1u >= args_len {
                        return ParseErr("Missing match pattern".to_owned());
                    }
                    pat.push(args[ix + 1u].clone());
                    ix += 2u;
                }
                "-builtins" => {
                    builtins = true;
                    ix += 1u;
                }
                "-abi" => {
                    abi = args[ix + 1u].clone();
                    ix += 2u;
                }
                "-allow-bitfields" => {
                  fail_on_bitfield = false;
                  ix += 1u;
                }
                _ => {
                    clang_args.push(args[ix].clone());
                    ix += 1u;
                }
            }
        }
    }

    let ctx = box BindGenCtx {
        match_pat: pat,
        abi: abi,
        builtins: builtins,
        links: links,
        name: HashMap::new(),
        globals: vec!(),
        builtin_defs: vec!(),
        builtin_names: builtin_names(),
        emit_ast: emit_ast,
        fail_on_bitfield: fail_on_bitfield
    };

    return ParseOk(clang_args, ctx, out);
}

fn builtin_names() -> HashSet<~str> {
    let mut names = HashSet::new();
    let keys = ~[
        "__va_list_tag".to_owned(),
        "__va_list".to_owned(),
    ];

    keys.move_iter().advance(|s| {
        names.insert(s);
        true
    });

    return names;
}

fn print_usage(bin: ~str) {
    io::stdio::print(format!("Usage: {} [options] input.h", bin) +
"
Options:
    -h or --help          Display help message
    -l <name> or -l<name> Name of a library to link to, can be proivded
                          multiple times
    -o <output.rs>        Write bindings to <output.rs> (default stdout)
    -match <name>         Only output bindings for definitions from files
                          whose name contains <name>
                          If multiple -match options are provided, files
                          matching any rule are bound to.
    -builtins             Output bindings for builtin definitions
                          (for example __builtin_va_list)
    -abi <abi>            Indicate abi of extern functions (default C)
    -allow-bitfields      Don't fail if we encounter a bitfield
                          (default is false, as rust doesn't support bitfields)
    -emit-clang-ast       Output the ast (for debugging purposes)

    Options other than stated above are passed to clang.
"
    );
}

fn match_pattern(ctx: &mut BindGenCtx, cursor: &Cursor) -> bool {
    let (file, _, _, _) = cursor.location().location();

    if file.is_null() {
        return ctx.builtins;
    }

    if ctx.match_pat.is_empty() {
        return true;
    }

    let name = file.name();
    let mut found = false;
    ctx.match_pat.iter().advance(|pat| {
        if name.contains(*pat) {
            found = true;
        }
        true
    });

    return found;
}

fn decl_name(ctx: &mut BindGenCtx, cursor: &Cursor) -> Global {
    let mut new_decl = false;
    let decl = {
        *ctx.name.find_or_insert_with(*cursor, |_| {
            new_decl = true;
            let spelling = cursor.spelling();
            let ty = cursor.cur_type();
            let layout = Layout::new(ty.size(), ty.align());

            match cursor.kind() {
              CXCursor_StructDecl => {
                let ci = @RefCell::new(CompInfo::new(spelling, true, vec!(), layout));
                GCompDecl(ci)
              }
              CXCursor_UnionDecl => {
                let ci = @RefCell::new(CompInfo::new(spelling, false, vec!(), layout));
                GCompDecl(ci)
              }
              CXCursor_EnumDecl => {
                let kind = match cursor.enum_type().kind() {
                    CXType_SChar | CXType_Char_S => ISChar,
                    CXType_UChar | CXType_Char_U => IUChar,
                    CXType_UShort => IUShort,
                    CXType_UInt => IUInt,
                    CXType_ULong => IULong,
                    CXType_ULongLong => IULongLong,
                    CXType_Short => IShort,
                    CXType_Int => IInt,
                    CXType_Long => ILong,
                    CXType_LongLong => ILongLong,
                    _ => IInt,
                };
                let ei = @RefCell::new(EnumInfo::new(spelling, kind, vec!(), layout));
                GEnumDecl(ei)
              }
              CXCursor_TypedefDecl => {
                let ti = @RefCell::new(TypeInfo::new(spelling, TVoid));
                GType(ti)
              }
              CXCursor_VarDecl => {
                let vi = @RefCell::new(VarInfo::new(spelling, TVoid));
                GVar(vi)
              }
              CXCursor_FunctionDecl => {
                let vi = @RefCell::new(VarInfo::new(spelling, TVoid));
                GFunc(vi)
              }
              _ => GOther
            }
        })
    };

    if new_decl {
        if ctx.builtin_names.contains(&cursor.spelling()) {
            ctx.builtin_defs.push(*cursor);
        }
    }

    return decl;
}

fn opaque_decl(ctx: &mut BindGenCtx, decl: &Cursor) {
    let name = decl_name(ctx, decl);
    ctx.globals.push(name);
}

fn fwd_decl(ctx: &mut BindGenCtx, cursor: &Cursor, f: |ctx: &mut BindGenCtx|) {
    let def = &cursor.definition();
    if cursor == def {
        f(ctx);
    } else if def.kind() == CXCursor_NoDeclFound ||
              def.kind() == CXCursor_InvalidFile {
        opaque_decl(ctx, cursor);
    }
}

fn conv_ptr_ty(ctx: &mut BindGenCtx, ty: &cx::Type, cursor: &Cursor, layout: Layout) -> il::Type {
    let is_const = ty.is_const();
    match ty.kind() {
      CXType_Void => {
        return TPtr(box TVoid, is_const, layout)
      }
      CXType_Unexposed |
      CXType_FunctionProto |
      CXType_FunctionNoProto => {
        let ret_ty = ty.ret_type();
        let decl = ty.declaration();
        return if ret_ty.kind() != CXType_Invalid {
            let args_lst = ty.arg_types().iter().map(|arg| {
                ("".to_owned(), conv_ty(ctx, arg, cursor))
            }).collect();
            let ret_ty = box conv_ty(ctx, &ret_ty, cursor);

            TFunc(ret_ty, args_lst, ty.is_variadic())
        } else if decl.kind() != CXCursor_NoDeclFound {
            TPtr(box conv_decl_ty(ctx, &decl), is_const, layout)
        } else {
            TPtr(box TVoid, is_const, layout)
        };
      }
      CXType_Typedef => {
        let decl = ty.declaration();
        let def_ty = decl.typedef_type();
        if def_ty.kind() == CXType_FunctionProto ||
           def_ty.kind() == CXType_FunctionNoProto {
            return TPtr(box conv_ptr_ty(ctx, &def_ty, cursor, layout), is_const, layout);
        } else {
            return TPtr(box conv_ty(ctx, ty, cursor), is_const, layout);
        }
      }
      _ => return TPtr(box conv_ty(ctx, ty, cursor), is_const, layout),
    }
}

fn conv_decl_ty(ctx: &mut BindGenCtx, cursor: &Cursor) -> il::Type {
    return match cursor.kind() {
      CXCursor_StructDecl => {
        let decl = decl_name(ctx, cursor);
        let ci = decl.compinfo();
        TComp(ci)
      }
      CXCursor_UnionDecl => {
        let decl = decl_name(ctx, cursor);
        let ci = decl.compinfo();
        TComp(ci)
      }
      CXCursor_EnumDecl => {
        let decl = decl_name(ctx, cursor);
        let ei = decl.enuminfo();
        TEnum(ei)
      }
      CXCursor_TypedefDecl => {
        let decl = decl_name(ctx, cursor);
        let ti = decl.typeinfo();
        TNamed(ti)
      }
      _ => TVoid
    };
}

fn conv_ty(ctx: &mut BindGenCtx, ty: &cx::Type, cursor: &Cursor) -> il::Type {
    debug!("conv_ty: ty=`{}`", type_to_str(ty.kind()));
    let layout = Layout::new(ty.size(), ty.align());
    return match ty.kind() {
      CXType_Void | CXType_Invalid => TVoid,
      CXType_Bool => TInt(IBool, layout),
      CXType_SChar |
      CXType_Char_S => TInt(ISChar, layout),
      CXType_UChar |
      CXType_Char_U => TInt(IUChar, layout),
      CXType_UShort => TInt(IUShort, layout),
      CXType_UInt => TInt(IUInt, layout),
      CXType_ULong => TInt(IULong, layout),
      CXType_ULongLong => TInt(IULongLong, layout),
      CXType_Short => TInt(IShort, layout),
      CXType_Int => TInt(IInt, layout),
      CXType_Long => TInt(ILong, layout),
      CXType_LongLong => TInt(ILongLong, layout),
      CXType_Float => TFloat(FFloat, layout),
      CXType_Double => TFloat(FDouble, layout),
      CXType_LongDouble => TFloat(FDouble, layout),
      CXType_Pointer => conv_ptr_ty(ctx, &ty.pointee_type(), cursor, layout),
      CXType_VariableArray | CXType_DependentSizedArray => {
          conv_ptr_ty(ctx, &ty.elem_type(), cursor, layout)
      }
      CXType_Record |
      CXType_Typedef  |
      CXType_Unexposed |
      CXType_Enum => conv_decl_ty(ctx, &ty.declaration()),
      CXType_ConstantArray => TArray(box conv_ty(ctx, &ty.elem_type(), cursor), ty.array_size(), layout),
      _ => fail!("unhandled type kind: `{}`", type_to_str(ty.kind())),
    };
}

fn opaque_ty(ctx: &mut BindGenCtx, ty: &cx::Type) {
    if ty.kind() == CXType_Record || ty.kind() == CXType_Enum {
        let decl = ty.declaration();
        let def = decl.definition();
        if def.kind() == CXCursor_NoDeclFound ||
           def.kind() == CXCursor_InvalidFile {
            opaque_decl(ctx, &decl);
        }
    }
}

fn visit_struct(cursor: &Cursor,
                parent: &Cursor,
                ctx: &mut BindGenCtx,
                fields: &mut Vec<FieldInfo>) -> Enum_CXVisitorResult {
    if cursor.kind() == CXCursor_FieldDecl {
        let ty = conv_ty(ctx, &cursor.cur_type(), cursor);
        let name = cursor.spelling();
        let bit = cursor.bit_width();
        // If we encounter a bitfield, and fail_on_bitfield is set, throw an
        // error and exit entirely.
        if bit != None && ctx.fail_on_bitfield {
            fail!("Cannot handle bitfield `{}` in struct `{}`",
                  name, parent.spelling());
        }
        let field = FieldInfo::new(name, ty, bit);
        fields.push(field);
    }
    return CXChildVisit_Continue;
}

fn visit_union(cursor: &Cursor,
               ctx: &mut BindGenCtx,
               fields: &mut Vec<FieldInfo>) -> Enum_CXVisitorResult {
    if cursor.kind() == CXCursor_FieldDecl {
        let ty = conv_ty(ctx, &cursor.cur_type(), cursor);
        let name = cursor.spelling();
        let field = FieldInfo::new(name, ty, None);
        fields.push(field);
    }
    return CXChildVisit_Continue;
}

fn visit_enum(cursor: &Cursor,
              items: &mut Vec<EnumItem>) -> Enum_CXVisitorResult {
    if cursor.kind() == CXCursor_EnumConstantDecl {
        let name = cursor.spelling();
        let val = cursor.enum_val();
        let item = EnumItem::new(name, val);
        items.push(item);
    }
    return CXChildVisit_Continue;
}

fn visit_top<'r>(cur: &'r Cursor,
                 parent: &'r Cursor,
                 ctx: &mut BindGenCtx) -> Enum_CXVisitorResult {
    let cursor = if ctx.builtin_names.contains(&parent.spelling()) {
        parent
    } else {
        cur
    };

    if !match_pattern(ctx, cursor) {
        return CXChildVisit_Continue;
    }

    match cursor.kind() {
      CXCursor_StructDecl => {
        fwd_decl(ctx, cursor, |ctx_| {
            let decl = decl_name(ctx_, cursor);
            let ci = decl.compinfo();
            cursor.visit(|c, p| {
                let mut ci_ = ci.borrow_mut();
                visit_struct(c, p, ctx_, &mut ci_.fields)
            });
            ctx_.globals.push(GComp(ci));
        });
        return if cur.kind() == CXCursor_FieldDecl {
            CXChildVisit_Break
        } else {
            CXChildVisit_Recurse
        };
      }
      CXCursor_UnionDecl => {
        fwd_decl(ctx, cursor, |ctx_| {
            let decl = decl_name(ctx_, cursor);
            let ci = decl.compinfo();
            cursor.visit(|c, _| {
                let mut ci_ = ci.borrow_mut();
                visit_union(c, ctx_, &mut ci_.fields)
            });
            ctx_.globals.push(GComp(ci));
        });
        return CXChildVisit_Recurse;
      }
      CXCursor_EnumDecl => {
        fwd_decl(ctx, cursor, |ctx_| {
            let decl = decl_name(ctx_, cursor);
            let ei = decl.enuminfo();
            cursor.visit(|c, _| {
                let mut ei_ = ei.borrow_mut();
                visit_enum(c, &mut ei_.items)
            });
            ctx_.globals.push(GEnum(ei));
        });
        return CXChildVisit_Continue;
      }
      CXCursor_FunctionDecl => {
        let linkage = cursor.linkage();
        if linkage != CXLinkage_External && linkage != CXLinkage_UniqueExternal {
            return CXChildVisit_Continue;
        }

        let args_lst: Vec<(~str, il::Type)> = cursor.args().iter().map(|arg| {
            let arg_name = arg.spelling();
            (arg_name, conv_ty(ctx, &arg.cur_type(), cursor))
        }).collect();

        let ty = cursor.cur_type();
        let ret_ty = box conv_ty(ctx, &cursor.ret_type(), cursor);

        let func = decl_name(ctx, cursor);
        let vi = func.varinfo();
        let mut vi = vi.borrow_mut();
        vi.ty = TFunc(ret_ty.clone(), args_lst.clone(), ty.is_variadic());
        ctx.globals.push(func);

        return CXChildVisit_Continue;
      }
      CXCursor_VarDecl => {
        let linkage = cursor.linkage();
        if linkage != CXLinkage_External && linkage != CXLinkage_UniqueExternal {
            return CXChildVisit_Continue;
        }

        let ty = conv_ty(ctx, &cursor.cur_type(), cursor);
        let var = decl_name(ctx, cursor);
        let vi = var.varinfo();
        let mut vi = vi.borrow_mut();
        vi.ty = ty.clone();
        vi.is_const = cursor.cur_type().is_const();
        ctx.globals.push(var);

        return CXChildVisit_Continue;
      }
      CXCursor_TypedefDecl => {
        let mut under_ty = cursor.typedef_type();
        if under_ty.kind() == CXType_Unexposed {
            under_ty = under_ty.canonical_type();
        }

        let ty = conv_ty(ctx, &under_ty, cursor);
        let typedef = decl_name(ctx, cursor);
        let ti = typedef.typeinfo();
        let mut ti = ti.borrow_mut();
        ti.ty = ty.clone();
        ctx.globals.push(typedef);

        opaque_ty(ctx, &under_ty);

        return CXChildVisit_Continue;
      }
      CXCursor_FieldDecl => {
        return CXChildVisit_Continue;
      }
      _ => return CXChildVisit_Recurse,
    }
}

#[main]
fn main() {
    let mut bind_args = os::args();
    let bin = bind_args.shift().unwrap();

    match parse_args(bind_args.as_slice()) {
        ParseErr(e) => fail!(e),
        CmdUsage => print_usage(bin),
        ParseOk(clang_args, mut ctx, out) => {
            let ix = cx::Index::create(false, true);
            if ix.is_null() {
                fail!("clang failed to create index".to_owned());
            }

            let unit = TranslationUnit::parse(&ix, "", clang_args, [], 0);
            if unit.is_null() {
                fail!("No input files given".to_owned());
            }

            let mut c_err = false;
            let diags = unit.diags();
            let stderr = &mut io::stderr() as &mut io::Writer;
            diags.iter().advance(|d| {
                let msg = d.format(Diagnostic::default_opts());
                stderr.write(msg.as_bytes());
                stderr.write(['\n' as u8]);
                if d.severity() >= CXDiagnostic_Error {
                    c_err = true;
                }
                true
            });

            if c_err {
                os::set_exit_status(1);
                return;
            }

            let cursor = unit.cursor();

            if ctx.emit_ast {
                cursor.visit(|cur, _| ast_dump(cur, 0));
            }			

            cursor.visit(|cur, parent| visit_top(cur, parent, ctx));

            while !ctx.builtin_defs.is_empty() {
                let c = ctx.builtin_defs.shift().unwrap();
                c.visit(|cur, parent| visit_top(cur, parent, ctx));
            }

            gen_rs(out, ctx.abi.clone(), ctx.links.as_slice(), ctx.globals.clone());

            unit.dispose();
            ix.dispose();
        }
    }
}
