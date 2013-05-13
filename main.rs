use core::hashmap::{HashMap, HashSet};
use core::io::WriterUtil;

use il = types;
use types::*;
use cx = clang;
use clang::*;
use clang::ll::*;
use gen::*;

struct BindGenCtx {
    match_pat: ~[~str],
    builtins: bool,
    link: Option<~str>,
    out: @io::Writer,
    name: HashMap<Cursor, Global>,
    globals: ~[Global],
    builtin_defs: ~[Cursor],
    builtin_names: HashSet<~str>
}

enum ParseResult {
    CmdUsage,
    ParseOk(~[~str], @mut BindGenCtx),
    ParseErr(~str)
}

fn parse_args(args: &[~str]) -> ParseResult {
    let mut clang_args = ~[];
    let args_len = vec::len(args);

    let mut out = io::stdout();
    let mut pat = ~[];
    let mut link = None;
    let mut builtins = false;

    if args_len == 0u {
        return CmdUsage;
    }

    let mut ix = 0u;
    while ix < args_len {
        match args[ix] {
            ~"--help" | ~"-h" => {
                return CmdUsage;
            }
            ~"-o" => {
                if ix + 1u > args_len {
                    return ParseErr(~"Missing output filename");
                }
                match io::file_writer(&path::Path(args[ix + 1u]),
                                      ~[io::Create, io::Truncate]) {
                  Ok(f) => { out = f; }
                  Err(e) => { return ParseErr(e); }
                }
                ix += 2u;
            }
            ~"-l" => {
                if ix + 1u > args_len {
                    return ParseErr(~"Missing link name");
                }
                link = Some(copy args[ix + 1u]);
                ix += 2u;
            }
            ~"-match" => {
                if ix + 1u > args_len {
                    return ParseErr(~"Missing match pattern");
                }
                pat.push(copy args[ix + 1u]);
                ix += 2u;
            }
            ~"-builtins" => {
                builtins = true;
                ix += 1u;
            }
            _ => {
                clang_args.push(copy args[ix]);
                ix += 1u;
            }
        }
    }

    let ctx = @mut BindGenCtx { match_pat: pat,
                                builtins: builtins,
                                link: link,
                                out: out,
                                name: HashMap::new(),
                                globals: ~[],
                                builtin_defs: ~[],
                                builtin_names: builtin_names()
                              };

    return ParseOk(clang_args, ctx);
}

fn builtin_names() -> HashSet<~str> {
    let mut names = HashSet::new();
    let keys = ~[
        ~"__va_list_tag"
    ];

    do vec::consume(keys) |_, s| {
        names.insert(s);
    }

    return names;
}

fn print_usage(bin: ~str) {
    io::print(fmt!("Usage: %s [options] input.h", bin) +
"
Options:
    -h or --help    Display help message
    -l <name>       Link name of the library
    -o <output.rs>  Write bindings to <output.rs> (default stdout)
    -match <name>   Only output bindings for definitions from files
                    whose name contains <name>
                    If multiple -match options are provided, files
                    matching any rule are bound to.
    -builtins       Output bindings for builtin definitions
                    (for example __builtin_va_list)

    Options other than stated above are passed to clang.
"
    );
}

fn match_pattern(ctx: @mut BindGenCtx, cursor: &Cursor) -> bool {
    let (file, _, _, _) = cursor.location().location();

    if file.is_null() {
        return ctx.builtins;
    }

    if ctx.match_pat.is_empty() {
        return true;
    }

    let name = file.name();
    for ctx.match_pat.each |pat| {
        if str::contains(name, *pat) {
            return true;
        }
    }

    return false;
}

fn decl_name(ctx: @mut BindGenCtx, cursor: &Cursor) -> Global {
    let mut new_decl = false;
    let decl = {
        *do ctx.name.find_or_insert_with(*cursor) |_| {
            new_decl = true;
            let spelling = cursor.spelling();
    
            let decl = match cursor.kind() {
              CXCursor_StructDecl => {
                let ci = mk_compinfo(spelling, true, ~[]);
                GCompDecl(ci)
              }
              CXCursor_UnionDecl => {
                let ci = mk_compinfo(spelling, false, ~[]);
                GCompDecl(ci)
              }
              CXCursor_EnumDecl => {
                let kind = if cursor.enum_type().kind() == CXType_Int {
                    IInt
                } else {
                    IUInt
                };
                let ei = mk_enuminfo(spelling, kind, ~[]);
                GEnumDecl(ei)
              }
              CXCursor_TypedefDecl => {
                let ti = mk_typeinfo(spelling, @TVoid);
                GType(ti)
              }
              CXCursor_VarDecl => {
                let vi = mk_varinfo(spelling, @TVoid);
                GVar(vi)
              }
              CXCursor_FunctionDecl => {
                let vi = mk_varinfo(spelling, @TVoid);
                GFunc(vi)
              }
              _ => GOther
            };
    
            decl
        }
    };

    if (new_decl) {
        if ctx.builtin_names.contains(&cursor.spelling()) {
            ctx.builtin_defs.push(*cursor);
        }
    }

    return decl;
}

fn opaque_decl(ctx: @mut BindGenCtx, decl: &Cursor) {
    let name = decl_name(ctx, decl);
    ctx.globals.push(name);
}

fn fwd_decl(ctx: @mut BindGenCtx, cursor: &Cursor, f: &fn()) {
    let def = &cursor.definition();
    if cursor == def {
        f();
    } else if def.kind() == CXCursor_NoDeclFound ||
              def.kind() == CXCursor_InvalidFile {
        opaque_decl(ctx, cursor);
    }
}

fn conv_ptr_ty(ctx: @mut BindGenCtx, ty: &cx::Type, cursor: &Cursor) -> @il::Type {
    let is_const = ty.is_const();
    match ty.kind() {
      CXType_Void => {
        return @TPtr(@TVoid, is_const)
      }
      CXType_Unexposed |
      CXType_FunctionProto |
      CXType_FunctionNoProto => {
        let ret_ty = ty.ret_type();
        let decl = ty.declaration();
        return if ret_ty.kind() != CXType_Invalid {
            let args_lst = do ty.arg_types().map |arg| {
                (~"", conv_ty(ctx, arg, cursor))
            };
            let ret_ty = conv_ty(ctx, &ret_ty, cursor);

            @TFunc(ret_ty, args_lst, ty.is_variadic())
        } else if decl.kind() != CXCursor_NoDeclFound {
            @TPtr(conv_decl_ty(ctx, &decl), is_const)
        } else {
            @TPtr(@TVoid, is_const)
        };
      }
      CXType_Typedef => {
        let decl = ty.declaration();
        let def_ty = decl.typedef_type();
        if def_ty.kind() == CXType_FunctionProto ||
           def_ty.kind() == CXType_FunctionNoProto {
            return @TPtr(conv_ptr_ty(ctx, &def_ty, cursor), is_const);
        } else {
            return @TPtr(conv_ty(ctx, ty, cursor), is_const);
        }
      }
      _ => return @TPtr(conv_ty(ctx, ty, cursor), is_const),
    }
}

fn conv_decl_ty(ctx: @mut BindGenCtx, cursor: &Cursor) -> @il::Type {
    return match cursor.kind() {
      CXCursor_StructDecl => {
        let decl = decl_name(ctx, cursor);
        let ci = global_compinfo(decl);
        @TComp(ci)
      }
      CXCursor_UnionDecl => {
        let decl = decl_name(ctx, cursor);
        let ci = global_compinfo(decl);
        @TComp(ci)
      }
      CXCursor_EnumDecl => {
        let decl = decl_name(ctx, cursor);
        let ei = global_enuminfo(decl);
        @TEnum(ei)
      }
      CXCursor_TypedefDecl => {
        let decl = decl_name(ctx, cursor);
        let ti = global_typeinfo(decl);
        @TNamed(ti)
      }
      _ => @TVoid
    };
}

fn conv_ty(ctx: @mut BindGenCtx, ty: &cx::Type, cursor: &Cursor) -> @il::Type {
    return match ty.kind() {
      CXType_Bool => @TInt(IBool),
      CXType_SChar |
      CXType_Char_S => @TInt(ISChar),
      CXType_UChar |
      CXType_Char_U => @TInt(IUChar),
      CXType_UShort => @TInt(IUShort),
      CXType_UInt => @TInt(IUInt),
      CXType_ULong => @TInt(IULong),
      CXType_ULongLong => @TInt(IULongLong),
      CXType_Short => @TInt(IShort),
      CXType_Int => @TInt(IInt),
      CXType_Long => @TInt(ILong),
      CXType_LongLong => @TInt(ILongLong),
      CXType_Float => @TFloat(FFloat),
      CXType_Double => @TFloat(FDouble),
      CXType_LongDouble => @TFloat(FDouble),
      CXType_Pointer => conv_ptr_ty(ctx, &ty.pointee_type(), cursor),
      CXType_Record |
      CXType_Typedef  |
      CXType_Unexposed |
      CXType_Enum => conv_decl_ty(ctx, &ty.declaration()),
      CXType_ConstantArray => @TArray(conv_ty(ctx, &ty.elem_type(), cursor), ty.array_size()),
      _ => @TVoid
    };
}

fn opaque_ty(ctx: @mut BindGenCtx, ty: &cx::Type) {
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
                ctx: @mut BindGenCtx,
                fields: &mut ~[@FieldInfo]) -> Enum_CXVisitorResult {
    if cursor.kind() == CXCursor_FieldDecl {
        let ty = conv_ty(ctx, &cursor.cur_type(), cursor);
        let name = cursor.spelling();
        let field = mk_fieldinfo(name, ty, None);
        fields.push(field);
    }
    return CXChildVisit_Continue;
}

fn visit_union(cursor: &Cursor,
               ctx: @mut BindGenCtx,
               fields: &mut ~[@FieldInfo]) -> Enum_CXVisitorResult {
    if cursor.kind() == CXCursor_FieldDecl {
        let ty = conv_ty(ctx, &cursor.cur_type(), cursor);
        let name = cursor.spelling();
        let field = mk_fieldinfo(name, ty, None);
        fields.push(field);
    }
    return CXChildVisit_Continue;
}

fn visit_enum(cursor: &Cursor,
              items: &mut ~[@EnumItem]) -> Enum_CXVisitorResult {
    if cursor.kind() == CXCursor_EnumConstantDecl {
        let name = cursor.spelling();
        let val = cursor.enum_val();
        let item = mk_enumitem(name, val);
        items.push(item);
    }
    return CXChildVisit_Continue;
}

fn visit_top<'r>(cur: &'r Cursor,
                 parent: &'r Cursor,
                 ctx: @mut BindGenCtx) -> Enum_CXVisitorResult {
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
        do fwd_decl(ctx, cursor) || {
            let decl = decl_name(ctx, cursor);
            let ci = global_compinfo(decl);
            cursor.visit(|c, _| visit_struct(c, ctx, &mut ci.fields));
            ctx.globals.push(GComp(ci));
        }
        return if cur.kind() == CXCursor_FieldDecl {
            CXChildVisit_Break
        } else {
            CXChildVisit_Recurse
        };
      }
      CXCursor_UnionDecl => {
        do fwd_decl(ctx, cursor) || {
            let decl = decl_name(ctx, cursor);
            let ci = global_compinfo(decl);
            cursor.visit(|c, _| visit_union(c, ctx, &mut ci.fields));
            ctx.globals.push(GComp(ci));
        }
        return CXChildVisit_Recurse;
      }
      CXCursor_EnumDecl => {
        do fwd_decl(ctx, cursor) || {
            let decl = decl_name(ctx, cursor);
            let ei = global_enuminfo(decl);
            cursor.visit(|c, _| visit_enum(c, &mut ei.items));
            ctx.globals.push(GEnum(ei));
        }
        return CXChildVisit_Continue;
      }
      CXCursor_FunctionDecl => {
        let linkage = cursor.linkage();
        if linkage != CXLinkage_External && linkage != CXLinkage_UniqueExternal {
            return CXChildVisit_Continue;
        }

        let args_lst = do cursor.args().map |arg| {
            let arg_name = arg.spelling();
            (arg_name, conv_ty(ctx, &arg.cur_type(), cursor))
        };

        let ty = cursor.cur_type();
        let ret_ty = conv_ty(ctx, &cursor.ret_type(), cursor);

        let func = decl_name(ctx, cursor);
        global_varinfo(func).ty = @TFunc(ret_ty, args_lst, ty.is_variadic());
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
        global_varinfo(var).ty = ty;
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
        global_typeinfo(typedef).ty = ty;
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
    let bin = bind_args.shift();

    match parse_args(bind_args) {
        ParseErr(e) => fail!(e),
        CmdUsage => print_usage(bin),
        ParseOk(clang_args, ctx) => {
            let ix = cx::Index::create(false, true);
            if ix.is_null() {
                fail!(~"clang failed to create index");
            }

            let unit = TranslationUnit::parse(&ix, "", clang_args, ~[], 0);
            if unit.is_null() {
                fail!(~"No input files given");
            }

            let mut c_err = false;
            for unit.diags().each |d: &Diagnostic| {
                io::stderr().write_line(d.format(Diagnostic::default_opts()));
                if d.severity() >= CXDiagnostic_Error {
                    c_err = true;
                }
            }

            if c_err {
                return;
            }

            let cursor = unit.cursor();
            cursor.visit(|cur, parent| visit_top(cur, parent, ctx));
                
            while !ctx.builtin_defs.is_empty() {
                let c = ctx.builtin_defs.shift();
                c.visit(|cur, parent| visit_top(cur, parent, ctx));
            }

            gen_rs(ctx.out, &ctx.link, ctx.globals);

            unit.dispose();
            ix.dispose();
        }
    }
}
