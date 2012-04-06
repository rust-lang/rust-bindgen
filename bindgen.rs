use std;
use rustsyntax;

import libc::*;

import std::map;
import map::hashmap;
import io::writer_util;

import rustsyntax::parse::parser::{bad_expr_word_table};

import clang::*;
import clang::bindgen::*;

type bind_ctx = {
    match: [str],
    link: str,
    out: io::writer,
    name: map::hashmap<CXCursor, str>,
    visited: map::hashmap<str, bool>,
    mut unnamed_ty: uint,
    mut unnamed_field: uint,
    keywords: hashmap<str, ()>
};

enum result {
    usage,
    ok([str], @bind_ctx),
    err(str)
}

fn CXCursor_hash(&&c: CXCursor) -> uint {
    ret clang_hashCursor(c) as uint;
}

fn CXCursor_eq(&&k1: CXCursor, &&k2: CXCursor) -> bool {
    ret clang_equalCursors(k1, k2) as int == 1;
}

fn parse_args(args: [str]) -> result {
    let mut clang_args = [];
    let args_len = vec::len(args);

    let mut out = io::stdout();
    let mut pat = [];
    let mut link = option::none;

    let mut ix = 0u;
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
           @{ match: pat,
             link: option::get(link),
             out: out,
             name: map::hashmap(CXCursor_hash, CXCursor_eq),
             visited: map::str_hash(),
             mut unnamed_ty: 0u,
             mut unnamed_field: 0u,
             keywords: bad_expr_word_table() });
}

fn print_usage(bin: str) {
    io::print(#fmt["Usage: %s [options] input.h", bin] +
"
Options:
    -l <name>       Link name of the library
    -o <output.rs>  Write bindings to <output.rs> (default stdout)
    -match <name>   Only output bindings for definitions from files
                    whose name contains <name>
                    If multiple -match options are provided, files
                    matching any rule are bound to.
"
    );
}

fn to_str(s: CXString) -> str unsafe {
    ret str::unsafe::from_c_str(clang_getCString(s));
}

fn match_pattern(ctx: @bind_ctx, cursor: CXCursor) -> bool {
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
    for vec::each(ctx.match) {|pat|
        if str::contains(name, pat) {
            ret true;
        }
    }

    ret false;
}

fn sym_visited(ctx: @bind_ctx, sym: str) -> bool {
    if ctx.visited.contains_key(sym) {
        ret true;
    }
    ctx.visited.insert(sym, true);
    ret false;
}

fn unnamed_name(ctx: @bind_ctx) -> str {
    ctx.unnamed_ty += 1u;
    ret "unnamed" + uint::str(ctx.unnamed_ty);
}

fn decl_name(ctx: @bind_ctx, cursor: CXCursor) -> str {
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

fn opaque_decl(ctx: @bind_ctx, decl: CXCursor) {
    let name = decl_name(ctx, decl);
    if !sym_visited(ctx, name) {
        ctx.out.write_line(#fmt["type %s = c_void;\n", name]);
    }
}

fn fwd_decl(ctx: @bind_ctx, cursor: CXCursor, f: fn()) {
    let def = clang_getCursorDefinition(cursor);
    if CXCursor_eq(cursor, def) {
        f();
    } else if def.kind == CXCursor_NoDeclFound ||
              def.kind == CXCursor_InvalidFile {
        opaque_decl(ctx, cursor);
    }
}

fn rust_id(ctx: @bind_ctx, name: str) -> str {
    if ctx.keywords.contains_key(name) {
        ret "_" + name;
    }
    ret name;
}

fn conv_ptr_ty(ctx: @bind_ctx, ty: CXType, cursor: CXCursor) -> str {
    if ty.kind == CXType_Void {
        ret "*c_void"
    } else if ty.kind == CXType_Unexposed ||
              ty.kind == CXType_FunctionProto ||
              ty.kind == CXType_FunctionNoProto {
        let ret_ty = clang_getResultType(ty);
        let decl = clang_getTypeDeclaration(ty);
        ret if ret_ty.kind != CXType_Invalid {
            "*u8"
        } else if decl.kind != CXCursor_NoDeclFound {
            "*" + conv_decl_ty(ctx, decl)
        } else {
            #fmt["*c_void /* unknown %s referenced by %s %s */",
                 to_str(clang_getTypeKindSpelling(ty.kind)),
                 to_str(clang_getCursorKindSpelling(cursor.kind)),
                 to_str(clang_getCursorSpelling(cursor))]
        }
    } else if ty.kind == CXType_Typedef {
        let decl = clang_getTypeDeclaration(ty);
        let def_ty = clang_getTypedefDeclUnderlyingType(decl);
        if def_ty.kind == CXType_FunctionProto ||
           def_ty.kind == CXType_FunctionNoProto {
            ret conv_ptr_ty(ctx, def_ty, cursor)
        }
    }
    ret "*" + conv_ty(ctx, ty, cursor)
}

fn conv_decl_ty(ctx: @bind_ctx, cursor: CXCursor) -> str {
    ret if cursor.kind == CXCursor_StructDecl ||
           cursor.kind == CXCursor_UnionDecl ||
           cursor.kind == CXCursor_EnumDecl {
        decl_name(ctx, cursor)
    } else if cursor.kind == CXCursor_TypedefDecl {
        rust_id(ctx, to_str(clang_getCursorSpelling(cursor)))
    } else {
        #fmt["c_void /* unknown %s %s */",
             to_str(clang_getCursorKindSpelling(cursor.kind)),
             to_str(clang_getCursorSpelling(cursor))]
    };
}

fn conv_ty(ctx: @bind_ctx, ty: CXType, cursor: CXCursor) -> str {
    ret if ty.kind == CXType_Bool {
        "bool"
    } else if ty.kind == CXType_SChar ||
              ty.kind == CXType_Char_S {
        "c_char"
    } else if ty.kind == CXType_UChar ||
              ty.kind == CXType_Char_U {
        "c_uchar"
    } else if ty.kind == CXType_UShort {
        "c_ushort"
    } else if ty.kind == CXType_UInt {
        "c_uint"
    } else if ty.kind == CXType_ULong {
        "c_ulong"
    } else if ty.kind == CXType_ULongLong {
        "c_ulonglong"
    } else if ty.kind == CXType_Short {
        "c_short"
    } else if ty.kind == CXType_Int {
        "c_int"
    } else if ty.kind == CXType_Long {
        "c_long"
    } else if ty.kind == CXType_LongLong {
        "c_longlong"
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

        if size == 0 {
            #fmt["/* FIXME: zero-sized array */\n"]
        } else if size == 1 {
            a_ty
        } else {
            let mut rust_ty = "(";
            let mut i = 1;
            while i < size {
                rust_ty += a_ty + ",";
                i += 1;
            }
            rust_ty += a_ty + ")";
            rust_ty
        }
    } else {
        #fmt["c_void /* unknown kind %s */",
             to_str(clang_getTypeKindSpelling(ty.kind))]
    };
}

fn opaque_ty(ctx: @bind_ctx, ty: CXType) {
    if ty.kind == CXType_Record || ty.kind == CXType_Enum {
        let decl = clang_getTypeDeclaration(ty);
        let def = clang_getCursorDefinition(decl);
        if def.kind == CXCursor_NoDeclFound || def.kind == CXCursor_InvalidFile {
            opaque_decl(ctx, decl);
        }
    }
}

crust fn visit_struct(++cursor: CXCursor,
                      ++_parent: CXCursor,
                      data: CXClientData) -> c_uint unsafe {
    let ctx = *(data as *@bind_ctx);
    if cursor.kind == CXCursor_FieldDecl {
        let ty = clang_getCursorType(cursor);
        let mut name = to_str(clang_getCursorSpelling(cursor));
        if str::is_empty(name) {
            name = "field_unnamed" + uint::str(ctx.unnamed_field);
            ctx.unnamed_field += 1u;
        }
        ctx.out.write_line(#fmt["    %s: %s,",
                                rust_id(ctx, name),
                                conv_ty(ctx, ty, cursor)]);
    }
    ret CXChildVisit_Continue;
}

crust fn visit_enum(++cursor: CXCursor,
                    ++parent: CXCursor,
                    data: CXClientData) -> c_uint unsafe {
    let ctx = *(data as *@bind_ctx);
    if cursor.kind == CXCursor_EnumConstantDecl {
        let int_ty =
            if clang_getEnumDeclIntegerType(parent).kind == CXType_Int {
                "i32"
            } else {
                "u32"
            };

        ctx.out.write_line(#fmt[
            "const %s: %s = %d_%s;",
            to_str(clang_getCursorSpelling(cursor)),
            int_ty,
            clang_getEnumConstantDeclValue(cursor),
            int_ty
        ]);
    }
    ret CXChildVisit_Continue;
}

crust fn visit_ty_top(++cursor: CXCursor,
                      ++_parent: CXCursor,
                      data: CXClientData) -> c_uint unsafe {
    let ctx = *(data as *@bind_ctx);
    if !match_pattern(ctx, cursor) {
        ret CXChildVisit_Continue;
    }

    if cursor.kind == CXCursor_StructDecl {
        fwd_decl(ctx, cursor) {||
            ctx.unnamed_field = 0u;
            ctx.out.write_line(#fmt["type %s = {",
                                    decl_name(ctx, cursor)]);
            clang_visitChildren(cursor, visit_struct, data);
            ctx.out.write_line("};\n");
        };
        ret CXChildVisit_Recurse;
    } else if cursor.kind == CXCursor_UnionDecl {
        fwd_decl(ctx, cursor) {||
            ctx.out.write_line(
                #fmt["type %s = c_void /* FIXME: union type */;\n",
                     decl_name(ctx, cursor)]
            );
        };
        ret CXChildVisit_Recurse;
    } else if cursor.kind == CXCursor_EnumDecl {
        fwd_decl(ctx, cursor) {||
            ctx.out.write_line(#fmt[
                "type %s = %s;", decl_name(ctx, cursor),
                conv_ty(ctx, clang_getEnumDeclIntegerType(cursor),
                        cursor)
            ]);
            clang_visitChildren(cursor, visit_enum, data);
        };
        ctx.out.write_line("");
        ret CXChildVisit_Continue;
    } else if cursor.kind == CXCursor_FunctionDecl {
            ret CXChildVisit_Continue;
    } else if cursor.kind == CXCursor_VarDecl {
        let name = to_str(clang_getCursorSpelling(cursor));
        if sym_visited(ctx, name) {
            ret CXChildVisit_Continue;
        }
        ctx.out.write_line(#fmt["/* FIXME: global variable %s */\n", name]);
        ret CXChildVisit_Continue;
    } else if cursor.kind == CXCursor_TypedefDecl {
        let name = to_str(clang_getCursorSpelling(cursor));
        if sym_visited(ctx, name) {
            ret CXChildVisit_Continue;
        }
        let mut under_ty = clang_getTypedefDeclUnderlyingType(cursor);
        if under_ty.kind == CXType_Unexposed {
            under_ty = clang_getCanonicalType(under_ty);
        }
        ctx.out.write_line(#fmt["type %s = %s;\n",
                                rust_id(ctx, name),
                                conv_ty(ctx, under_ty, cursor)]);
        opaque_ty(ctx, under_ty);
        ret CXChildVisit_Continue;
    } else if cursor.kind == CXCursor_FieldDecl {
        ret CXChildVisit_Continue;
    }

    ret CXChildVisit_Recurse;
}

crust fn visit_func_top(++cursor: CXCursor,
                        ++_parent: CXCursor,
                        data: CXClientData) -> c_uint unsafe {
    let ctx = *(data as *@bind_ctx);
    if !match_pattern(ctx, cursor) {
        ret CXChildVisit_Continue;
    }

    let linkage = clang_getCursorLinkage(cursor);
    if linkage != CXLinkage_External && linkage != CXLinkage_UniqueExternal {
        ret CXChildVisit_Continue;
    }

    if cursor.kind == CXCursor_FunctionDecl {
        let name = to_str(clang_getCursorSpelling(cursor));
        if sym_visited(ctx, name) {
            ret CXChildVisit_Continue;
        }

        let ty = clang_getCursorType(cursor);
        ctx.out.write_str(#fmt["fn %s(", rust_id(ctx, name)]);
        let arg_n = clang_getNumArgTypes(ty) as int;
        let mut i = 0;
        while i < arg_n {
            if i > 0 {
                ctx.out.write_str(", ");
            }
            let arg_ty = clang_getArgType(ty, i as c_uint);
            ctx.out.write_str(#fmt["++arg%d: %s",
                                    i, conv_ty(ctx, arg_ty, cursor)]);
            i += 1;
        }
        if clang_isFunctionTypeVariadic(ty) as uint != 0u {
            ctx.out.write_str("/* FIXME: variadic arguments */");
        }
        ctx.out.write_str(")");
        let ret_ty = clang_getCursorResultType(cursor);
        if ret_ty.kind != CXType_Void {
            ctx.out.write_str(#fmt[" -> %s",
                                    conv_ty(ctx, ret_ty, cursor)]);
        }
        ctx.out.write_line(";\n");
        ret CXChildVisit_Continue;
    }

    ret CXChildVisit_Recurse;
}

fn main(args: [str]) unsafe {
    let mut bind_args = args;
    let bin = vec::shift(bind_args);

    alt parse_args(bind_args) {
        err(e) { fail e; }
        usage { print_usage(bin); }
        ok(clang_args, ctx) {
            let ix = clang_createIndex(0 as c_int, 1 as c_int);
            if ix as int == 0 {
                fail "clang failed to create index";
            }

            let c_args = vec::map(clang_args, {|s|
                str::as_c_str(s, {|b| b })
            });
            let unit = clang_parseTranslationUnit(
                ix, ptr::null(),
                vec::unsafe::to_ptr(c_args),
                vec::len(c_args) as c_int,
                ptr::null(),
                0 as c_uint, 0 as c_uint
            );
            if unit as int == 0 {
                fail "No input files given";
            }

            let mut c_err = false;
            let mut i = 0u;
            let diag_num = clang_getNumDiagnostics(unit) as uint;
            while i < diag_num {
                let diag = clang_getDiagnostic(unit, i as c_uint);
                io::stderr().write_line(to_str(clang_formatDiagnostic(
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
            ctx.out.write_line("import libc::*;\n");
            clang_visitChildren(cursor, visit_ty_top,
                                ptr::addr_of(ctx) as CXClientData);
            ctx.out.write_line(#fmt["#[link_name=\"%s\"]", ctx.link]);
            ctx.out.write_line("native mod bindgen {\n");
            clang_visitChildren(cursor, visit_func_top,
                                ptr::addr_of(ctx) as CXClientData);
            ctx.out.write_line("}");

            clang_disposeTranslationUnit(unit);
            clang_disposeIndex(ix);
        }
    }
}
