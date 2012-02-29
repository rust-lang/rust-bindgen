use std;

import ctypes::*;

import std::io;
import std::os;
import std::map;
import io::writer_util;

import clang::*;
import clang::bindgen::*;

// workaround
import clang::{CXType_FunctionProto, CXType_FunctionNoProto,
               CXType_Record, CXType_Enum,
               CXType_Unexposed};

type bind_ctx = {
    match: [str],
    link: str,
    out: io::writer,
    name: map::map<CXCursor, str>,
    visited: map::map<str, bool>,
    mutable unnamed_ty: uint,
    mutable unnamed_field: uint,
    keywords: [str]
};

enum result {
    usage,
    ok([str], bind_ctx),
    err(str)
}

fn CXCursor_hash(&&c: CXCursor) -> uint {
    ret clang_hashCursor(c) as uint;
}

fn CXCursor_eq(&&k1: CXCursor, &&k2: CXCursor) -> bool {
    ret clang_equalCursors(k1, k2) as int == 0;
}

fn rust_keywords() -> [str] {
    ret [
        "alt", "assert", "be", "break", "check", "claim",
        "class", "const", "cont", "copy", "do", "else", "enum",
        "export", "fail", "fn", "for", "if",  "iface", "impl",
        "import", "let", "log", "mod", "mutable", "native", "pure",
        "resource", "ret", "trait", "type", "unchecked", "unsafe",
        "while", "crust", "mut"
    ];
}

fn parse_args(args: [str]) -> result {
    let clang_args = [];
    let args_len = vec::len(args);

    let out = io::stdout();
    let pat = [];
    let link = option::none;

    let ix = 0u;
    while ix < args_len {
        alt args[ix] {
            "--help" | "-h" {
                ret usage;
            }
            "-o" {
                if ix + 1u > args_len {
                    ret err("Missing output filename");
                }
                alt io::file_writer(args[ix + 1u],
                                    [io::create, io::truncate]) {
                    result::ok(f) { out = f; }
                    result::err(e) { ret err(e); }
                }
                ix += 2u;
            }
            "-l" {
                if ix + 1u > args_len {
                    ret err("Missing link name");
                }
                link = option::some(args[ix + 1u]);
                ix += 2u;
            }
            "-match" {
                if ix + 1u > args_len {
                    ret err("Missing match pattern");
                }
                vec::push(pat, args[ix + 1u]);
                ix += 2u;
            }
            _ {
                vec::push(clang_args, args[ix]);
                ix += 1u;
            }
        }
    }

    if option::is_none(link) {
        ret err("Link name is required");
    }

    ret ok(clang_args,
           { match: pat,
             link: option::get(link),
             out: out,
             name: map::mk_hashmap(CXCursor_hash, CXCursor_eq),
             visited: map::new_str_hash(),
             mutable unnamed_ty: 0u,
             mutable unnamed_field: 0u,
             keywords: rust_keywords() });
}

fn print_usage(bin: str) {
    io::print(#fmt("Usage: %s [options] input.h", bin) +
"
Options:
    -l <name>       Link name
    -o <output.rs>  Write bindings to <output.rs> (default stdout)
    -match <name>   Only output bindings for definitions from files
                    whose name contains <name>
                    If multiple -match options are provided, files
                    matching any rule are bound to.
"
    );
}

fn to_str(s: CXString) -> str {
    ret str::from_cstr(clang_getCString(s));
}

fn match_pattern(ctx: bind_ctx, cursor: CXCursor) -> bool {
    let file = ptr::null();
    clang_getSpellingLocation(clang_getCursorLocation(cursor),
                              ptr::addr_of(file),
                              ptr::null(), ptr::null(), ptr::null());

    if file as int == 0 {
        ret false;
    }

    if vec::is_empty(ctx.match) {
        ret true;
    }

    let name = to_str(clang_getFileName(file));
    for pat in ctx.match {
        if str::contains(pat, name) {
            ret true;
        }
    }

    ret false;
}

fn sym_visited(ctx: bind_ctx, sym: str) -> bool {
    if ctx.visited.contains_key(sym) {
        ret true;
    }
    ctx.visited.insert(sym, true);
    ret false;
}

fn unnamed_name(ctx: bind_ctx) -> str {
    ctx.unnamed_ty += 1u;
    ret "unnamed" + uint::str(ctx.unnamed_ty);
}

fn decl_name(ctx: bind_ctx, cursor: CXCursor) -> str {
    let name = ctx.name.find(cursor);
    alt name {
        option::some(n) { ret n; }
        none {
            let spelling = to_str(clang_getCursorSpelling(cursor));
            let prefix = if cursor.kind == CXCursor_StructDecl {
                "struct_"
            } else if cursor.kind == CXCursor_UnionDecl {
                "union_"
            } else if cursor.kind == CXCursor_EnumDecl {
                "enum_"
            } else {
                "other_"
            };
            let ty_name = if str::is_empty(spelling) {
                prefix + unnamed_name(ctx)
            } else {
                prefix + spelling
            };

            ctx.name.insert(cursor, ty_name);
            ret ty_name;
        }
    }
}

fn opaque_decl(ctx: bind_ctx, decl: CXCursor) {
    let name = decl_name(ctx, decl);
    if !sym_visited(ctx, name) {
        ctx.out.write_line(#fmt("type %s = void;\n", name));
    }
}

fn fwd_decl(ctx: bind_ctx, cursor: CXCursor, f: fn()) {
    let def = clang_getCursorDefinition(cursor);
    if cursor == def {
        f();
    } else if def.kind == CXCursor_NoDeclFound ||
              def.kind == CXCursor_InvalidFile {
        opaque_decl(ctx, def);
    }
}

fn rust_id(ctx: bind_ctx, name: str) -> str {
    if option::is_some(vec::find(ctx.keywords, {|k| k == name})) {
        ret "_" + name;
    }
    ret name;
}

fn conv_ptr_ty(ctx: bind_ctx, ty: CXType, cursor: CXCursor) -> str {
    ret if ty.kind == CXType_Void {
        "*void"
    } else if ty.kind == CXType_Unexposed ||
              ty.kind == CXType_FunctionProto ||
              ty.kind == CXType_FunctionNoProto {
        let ret_ty = clang_getResultType(ty);
        let decl = clang_getTypeDeclaration(ty);
        if ret_ty.kind != CXType_Invalid {
            "*u8"
        } else if decl.kind != CXCursor_NoDeclFound {
            "*" + conv_decl_ty(ctx, decl)
        } else {
            #fmt("*void /* unknown %s referenced by %s %s */",
                 to_str(clang_getTypeKindSpelling(ty.kind)),
                 to_str(clang_getCursorKindSpelling(cursor.kind)),
                 to_str(clang_getCursorSpelling(cursor)))
        }
    } else if ty.kind == CXType_Typedef {
        let decl = clang_getTypeDeclaration(ty);
        let def_ty = clang_getTypedefDeclUnderlyingType(decl);
        if def_ty.kind == CXType_FunctionProto ||
           def_ty.kind == CXType_FunctionNoProto {
            conv_ptr_ty(ctx, def_ty, cursor)
        } else {
            "*void"
        }
    } else  {
        "*" + conv_ty(ctx, ty, cursor)
    };
}

fn conv_decl_ty(ctx: bind_ctx, cursor: CXCursor) -> str {
    ret if cursor.kind == CXCursor_StructDecl ||
           cursor.kind == CXCursor_UnionDecl ||
           cursor.kind == CXCursor_EnumDecl {
        decl_name(ctx, cursor)
    } else if cursor.kind == CXCursor_TypedefDecl {
        rust_id(ctx, to_str(clang_getCursorSpelling(cursor)))
    } else {
        #fmt("void /* unknown %s %s */",
             to_str(clang_getCursorKindSpelling(clang_getCursorKind(cursor))),
             to_str(clang_getCursorSpelling(cursor)))
    };
}

fn conv_ty(ctx: bind_ctx, ty: CXType, cursor: CXCursor) -> str {
    ret if ty.kind == CXType_Bool {
        "bool"
    } else if ty.kind == CXType_SChar ||
              ty.kind == CXType_Char_S {
        "u8"
    } else if ty.kind == CXType_UChar ||
              ty.kind == CXType_Char_U {
        "u8"
    } else if ty.kind == CXType_UShort {
        "u16"
    } else if ty.kind == CXType_UInt {
        "c_uint"
    } else if ty.kind == CXType_ULong {
        "ulong"
    } else if ty.kind == CXType_ULongLong {
        "ulonglong"
    } else if ty.kind == CXType_Short {
        "i16"
    } else if ty.kind == CXType_Int {
        "c_iint"
    } else if ty.kind == CXType_Long {
        "long"
    } else if ty.kind == CXType_LongLong {
        "longlong"
    } else if ty.kind == CXType_Float {
        "c_float"
    } else if ty.kind == CXType_Double {
        "c_double"
    } else if ty.kind == CXType_Pointer {
            conv_ptr_ty(ctx, clang_getPointeeType(ty), cursor)
    } else if ty.kind == CXType_Record ||
              ty.kind == CXType_Typedef  ||
              ty.kind == CXType_Unexposed ||
              ty.kind == CXType_Enum {
        conv_decl_ty(ctx, clang_getTypeDeclaration(ty))
    } else if ty.kind == CXType_ConstantArray {
        let a_ty = conv_ty(ctx, clang_getArrayElementType(ty), cursor);
        let size = clang_getArraySize(ty) as int;
        let rust_ty = "(";
        let i = 0;
        while i < size {
            rust_ty += a_ty + ","
        }
        rust_ty += ")";
        rust_ty
    } else {
        #fmt("/* unknown kind %s */ void",
             to_str(clang_getTypeKindSpelling(ty.kind)))
    };
}

fn opaque_ty(ctx: bind_ctx, ty: CXType) {
    if ty.kind == CXType_Record || ty.kind == CXType_Enum {
        let decl = clang_getTypeDeclaration(ty);
        let def = clang_getCursorDefinition(decl);
        if def.kind == CXCursor_NoDeclFound || def.kind == CXCursor_InvalidFile {
            opaque_decl(ctx, decl);
        }
    }
}

crust fn visit_struct(++cursor: CXCursor,
                      ++parent: CXCursor,
                      data: CXClientData) -> c_int unsafe {
    let ctx = *(data as *bind_ctx);
    let kind = clang_getCursorKind(cursor);
    if kind == CXCursor_FieldDecl {
        let ty = clang_getCursorType(cursor);
        let name = to_str(clang_getCursorSpelling(cursor));
        if str::is_empty(name) {
            name = "field_unnamed" + uint::str(ctx.unnamed_field);
            ctx.unnamed_field += 1u;
        }
        ctx.out.write_line(#fmt("    %s: %s,",
                                rust_id(ctx, name),
                                conv_ty(ctx, ty, cursor)));
    }
    ret CXChildVisit_Continue;
}

crust fn visit_enum(++cursor: CXCursor,
                    ++parent: CXCursor,
                    data: CXClientData) -> c_int unsafe {
    let ctx = *(data as *bind_ctx);
    if cursor.kind == CXCursor_EnumConstantDecl {
        ctx.out.write_line(#fmt(
            "const %s: %s = %d_i32;",
            to_str(clang_getCursorSpelling(cursor)),
            conv_ty(ctx, clang_getEnumDeclIntegerType(parent), parent),
            clang_getEnumConstantDeclValue(cursor)
        ));
    }
    ret CXChildVisit_Continue;
}

crust fn visit_ty_top(++cursor: CXCursor,
                      ++parent: CXCursor,
                      data: CXClientData) -> c_int unsafe {
    let ctx = *(data as *bind_ctx);
    if !match_pattern(ctx, cursor) {
        ret CXChildVisit_Continue;
    }

    let kind = clang_getCursorKind(cursor);
    if kind == CXCursor_StructDecl {
        fwd_decl(ctx, cursor, {||
            ctx.unnamed_field = 0u;
            ctx.out.write_line(#fmt("type %s = {",
                                    decl_name(ctx, cursor)));
            clang_visitChildren(cursor, visit_struct, data);
            ctx.out.write_line("};\n");
        });
        ret CXChildVisit_Recurse;
    } else if kind == CXCursor_UnionDecl {
        fwd_decl(ctx, cursor, {||
            ctx.out.write_line(#fmt("type %s = void /* FIXME */ ;\n",
                                    decl_name(ctx, cursor)));
        });
        ret CXChildVisit_Recurse;
    } else if kind == CXCursor_EnumDecl {
        fwd_decl(ctx, cursor, {||
            ctx.out.write_line(#fmt(
                "type %s = %s;", decl_name(ctx, cursor),
                conv_ty(ctx, clang_getEnumDeclIntegerType(cursor),
                        cursor)
            ));
            clang_visitChildren(cursor, visit_enum, data);
        });
        ctx.out.write_line("");
        ret CXChildVisit_Continue;
    } else if kind == CXCursor_FunctionDecl {
            ret CXChildVisit_Continue;
    } else if kind == CXCursor_VarDecl {
        let name = to_str(clang_getCursorSpelling(cursor));
        if sym_visited(ctx, name) {
            ret CXChildVisit_Continue;
        }
        ctx.out.write_line(#fmt("/* FIXME: global var %s */", name));
        ret CXChildVisit_Continue;
    } else if kind == CXCursor_TypedefDecl {
        let name = to_str(clang_getCursorSpelling(cursor));
        if sym_visited(ctx, name) {
            ret CXChildVisit_Continue;
        }
        let under_ty = clang_getTypedefDeclUnderlyingType(cursor);
        if under_ty.kind == CXType_Unexposed {
            under_ty = clang_getCanonicalType(under_ty);
        }
        ctx.out.write_line(#fmt("type %s = %s;\n",
                                rust_id(ctx, name),
                                conv_ty(ctx, under_ty, cursor)));
        opaque_ty(ctx, under_ty);
        ret CXChildVisit_Continue;
    } else if kind == CXCursor_FieldDecl {
        ret CXChildVisit_Continue;
    }

    ret CXChildVisit_Recurse;
}

crust fn visit_func_top(++cursor: CXCursor,
                        ++parent: CXCursor,
                        data: CXClientData) -> c_int unsafe {
    let ctx = *(data as *bind_ctx);
    if !match_pattern(ctx, cursor) {
        ret CXChildVisit_Continue;
    }

    if clang_getCursorKind(cursor) == CXCursor_FunctionDecl {
        ret CXChildVisit_Continue;
    }

    ret CXChildVisit_Recurse;
}

fn main(args: [str]) unsafe {
    alt parse_args(args) {
        err(e) { fail e; }
        usage { print_usage(args[0]); }
        ok(clang_args, ctx) {
            let ix = clang_createIndex(0 as c_int, 1 as c_int);
            if ix as int == 0 {
                fail "clang failed to create index";
            }

            let c_args = vec::map(clang_args, {|s|
                str::as_buf(s, {|b| b })
            });
            let unit = clang_parseTranslationUnit(
                ix, ptr::null(),
                vec::unsafe::to_ptr(c_args),
                vec::len(c_args) as c_int,
                ptr::null(),
                0 as unsigned, 0 as unsigned
            );
            if unit as int == 0 {
                fail "No input files given";
            }

            let c_err = false;
            let i = 0u;
            let diag_num = clang_getNumDiagnostics(unit) as uint;
            while i < diag_num {
                let diag = clang_getDiagnostic(unit, i as unsigned);
                io::stderr().write_str(to_str(clang_formatDiagnostic(
                    diag, clang_defaultDiagnosticDisplayOptions()
                )));

                if clang_getDiagnosticSeverity(diag) >= CXDiagnostic_Error {
                    c_err = true
                }

                i += 1u;
            }

            if c_err {
                ret;
            }

            ctx.out.write_line(
                "/* automatically generated by rust-bindgen */\n"
            );

            let cursor = clang_getTranslationUnitCursor(unit);
            ctx.out.write_line("import ctypes::*;\n");
            clang_visitChildren(cursor, visit_ty_top,
                                ptr::addr_of(ctx) as CXClientData);
            ctx.out.write_line(#fmt("#[link_name=\"%s\"]", ctx.link));
            ctx.out.write_line("native mod bindgen {\n");
            clang_visitChildren(cursor, visit_func_top,
                                ptr::addr_of(ctx) as CXClientData);
            ctx.out.write_line("}");

            clang_disposeTranslationUnit(unit);
            clang_disposeIndex(ix);
        }
    }
}
