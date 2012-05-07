import std::map;
import map::hashmap;
import io::WriterUtil;

import libc::*;
import clang::*;
import clang::bindgen::*;

type bind_ctx = {
    match: ~[~str],
    link: ~str,
    out: io::Writer,
    name: map::hashmap<CXCursor, ~str>,
    visited: map::hashmap<~str, bool>,
    mut unnamed_ty: uint,
    mut unnamed_field: uint,
    keywords: hashmap<~str, ()>
};

enum result {
    usage,
    ok(~[~str], @bind_ctx),
    err(~str)
}

fn keyword_table() -> hashmap<~str, ()> {
    let words = map::str_hash();
    let keys = ~[
        ~"as",
        ~"else",
        ~"move",
        ~"of",
        ~"priv", ~"pub",
        ~"self", ~"send", ~"static",
        ~"to",
        ~"use",
        ~"with",

        ~"again", ~"assert",
        ~"break",
        ~"check", ~"const", ~"copy",
        ~"do", ~"drop",
        ~"else", ~"enum", ~"export", ~"extern",
        ~"fail", ~"false", ~"fn", ~"for",
        ~"if", ~"impl", ~"import",
        ~"let", ~"log", ~"loop",
        ~"match", ~"mod", ~"module", ~"move", ~"mut",
        ~"new",
        ~"owned",
        ~"pure",
        ~"ref", ~"return",
        ~"struct",
        ~"true", ~"trait", ~"type",
        ~"unchecked", ~"unsafe",
        ~"while"
    ];
    for keys.each |word| {
        words.insert(word, ());
    }
    return words;
}

fn CXCursor_hash(c: &CXCursor) -> uint {
    return clang_hashCursor(*c) as uint;
}

fn CXCursor_eq(k1: &CXCursor, k2: &CXCursor) -> bool {
    return clang_equalCursors(*k1, *k2) as int == 1;
}

fn parse_args(args: ~[~str]) -> result {
    let mut clang_args = ~[];
    let args_len = vec::len(args);

    let mut out = io::stdout();
    let mut pat = ~[];
    let mut link = ~"";

    if args_len == 0u {
        return usage;
    }

    let mut ix = 0u;
    while ix < args_len {
        match args[ix] {
            ~"--help" | ~"-h" => {
                return usage;
            }
            ~"-o" => {
                if ix + 1u > args_len {
                    return err(~"Missing output filename");
                }
                match io::file_writer(args[ix + 1u],
                                    ~[io::Create, io::Truncate]) {
                    result::ok(f) => { out = f; }
                    result::err(e) => { return err(e); }
                }
                ix += 2u;
            }
            ~"-l" => {
                if ix + 1u > args_len {
                    return err(~"Missing link name");
                }
                link = args[ix + 1u];
                ix += 2u;
            }
            ~"-match" => {
                if ix + 1u > args_len {
                    return err(~"Missing match pattern");
                }
                vec::push(pat, args[ix + 1u]);
                ix += 2u;
            }
            _ => {
                vec::push(clang_args, args[ix]);
                ix += 1u;
            }
        }
    }

    return ok(clang_args,
           @{ match: pat,
             link: link,
             out: out,
             name: map::hashmap(CXCursor_hash, CXCursor_eq),
             visited: map::str_hash(),
             mut unnamed_ty: 0u,
             mut unnamed_field: 0u,
             keywords: keyword_table() });
}

fn print_usage(bin: ~str) {
    io::print(#fmt["Usage: %s [options] input.h", bin] +
"
Options:
    -h or --help    Display help message
    -l <name>       Link name of the library
    -o <output.rs>  Write bindings to <output.rs> (default stdout)
    -match <name>   Only output bindings for definitions from files
                    whose name contains <name>
                    If multiple -match options are provided, files
                    matching any rule are bound to.

    Options other than stated above are passed to clang.
"
    );
}

fn to_str(s: CXString) -> ~str unsafe {
    return str::unsafe::from_c_str(clang_getCString(s));
}

fn match_pattern(ctx: @bind_ctx, cursor: CXCursor) -> bool {
    let file = ptr::null();
    clang_getSpellingLocation(clang_getCursorLocation(cursor),
                              ptr::addr_of(file),
                              ptr::null(), ptr::null(), ptr::null());

    if file as int == 0 {
        return false;
    }

    if vec::is_empty(ctx.match) {
        return true;
    }

    let name = to_str(clang_getFileName(file));
    for vec::each(ctx.match) |pat| {
        if str::contains(name, pat) {
            return true;
        }
    }

    return false;
}

fn sym_visited(ctx: @bind_ctx, sym: ~str) -> bool {
    if ctx.visited.contains_key(sym) {
        return true;
    }
    ctx.visited.insert(sym, true);
    return false;
}

fn unnamed_name(ctx: @bind_ctx) -> ~str {
    ctx.unnamed_ty += 1u;
    return ~"unnamed" + uint::str(ctx.unnamed_ty);
}

fn decl_name(ctx: @bind_ctx, cursor: CXCursor) -> ~str {
    let name = ctx.name.find(cursor);
    match name {
        option::some(n) => { return n; }
        none => {
            let spelling = to_str(clang_getCursorSpelling(cursor));
            let prefix = if cursor.kind == CXCursor_StructDecl {
                ~"struct_"
            } else if cursor.kind == CXCursor_UnionDecl {
                ~"union_"
            } else if cursor.kind == CXCursor_EnumDecl {
                ~"enum_"
            } else {
                ~"other_"
            };
            let ty_name = if str::is_empty(spelling) {
                prefix + unnamed_name(ctx)
            } else {
                prefix + spelling
            };

            ctx.name.insert(cursor, ty_name);
            return ty_name;
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
    if CXCursor_eq(&cursor, &def) {
        f();
    } else if def.kind == CXCursor_NoDeclFound ||
              def.kind == CXCursor_InvalidFile {
        opaque_decl(ctx, cursor);
    }
}

fn rust_id(ctx: @bind_ctx, name: ~str) -> ~str {
    if ctx.keywords.contains_key(name) {
        return ~"_" + name;
    }
    return name;
}

fn conv_ptr_ty(ctx: @bind_ctx, ty: CXType, cursor: CXCursor,
               opaque: bool) -> ~str {
    if ty.kind == CXType_Void {
        return ~"*c_void"
    } else if ty.kind == CXType_Unexposed ||
              ty.kind == CXType_FunctionProto ||
              ty.kind == CXType_FunctionNoProto {
        let ret_ty = clang_getResultType(ty);
        let decl = clang_getTypeDeclaration(ty);
        return if ret_ty.kind != CXType_Invalid {
            ~"*u8"
        } else if decl.kind != CXCursor_NoDeclFound {
            ~"*" + conv_decl_ty(ctx, decl, opaque)
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
            return conv_ptr_ty(ctx, def_ty, cursor, opaque)
        }
    }
    return ~"*" + conv_ty(ctx, ty, cursor, opaque)
}

fn conv_decl_ty(ctx: @bind_ctx, cursor: CXCursor, opaque: bool) -> ~str {
    return if cursor.kind == CXCursor_StructDecl {
        let name = decl_name(ctx, cursor);
        if opaque {
            #fmt["c_void /* %s */", name]
        } else {
            name
        }
    } else if cursor.kind == CXCursor_UnionDecl ||
            cursor.kind == CXCursor_EnumDecl {
        decl_name(ctx, cursor)
    } else if cursor.kind == CXCursor_TypedefDecl {
        let name = to_str(clang_getCursorSpelling(cursor));
        let ty = clang_getCanonicalType(
            clang_getTypedefDeclUnderlyingType(cursor)
        );
        if opaque &&
           ty.kind == CXType_Pointer &&
           clang_getPointeeType(ty).kind == CXType_Record {
            #fmt["*c_void /* %s */", name]
        } else {
            rust_id(ctx, name)
        }
    } else {
        #fmt["c_void /* unknown %s %s */",
             to_str(clang_getCursorKindSpelling(cursor.kind)),
             to_str(clang_getCursorSpelling(cursor))]
    };
}

fn conv_ty(ctx: @bind_ctx, ty: CXType, cursor: CXCursor,
           opaque: bool) -> ~str {
    return if ty.kind == CXType_Bool {
        ~"bool"
    } else if ty.kind == CXType_SChar ||
              ty.kind == CXType_Char_S {
        ~"c_char"
    } else if ty.kind == CXType_UChar ||
              ty.kind == CXType_Char_U {
        ~"c_uchar"
    } else if ty.kind == CXType_UShort {
        ~"c_ushort"
    } else if ty.kind == CXType_UInt {
        ~"c_uint"
    } else if ty.kind == CXType_ULong {
        ~"c_ulong"
    } else if ty.kind == CXType_ULongLong {
        ~"c_ulonglong"
    } else if ty.kind == CXType_Short {
        ~"c_short"
    } else if ty.kind == CXType_Int {
        ~"c_int"
    } else if ty.kind == CXType_Long {
        ~"c_long"
    } else if ty.kind == CXType_LongLong {
        ~"c_longlong"
    } else if ty.kind == CXType_Float {
        ~"c_float"
    } else if ty.kind == CXType_Double {
        ~"c_double"
    } else if ty.kind == CXType_Pointer {
        conv_ptr_ty(ctx, clang_getPointeeType(ty), cursor, opaque)
    } else if ty.kind == CXType_Record ||
              ty.kind == CXType_Typedef  ||
              ty.kind == CXType_Unexposed ||
              ty.kind == CXType_Enum {
        conv_decl_ty(ctx, clang_getTypeDeclaration(ty),
                     opaque && ty.kind == CXType_Typedef)
    } else if ty.kind == CXType_ConstantArray {
        let a_ty = conv_ty(ctx, clang_getArrayElementType(ty), cursor, opaque);
        let size = clang_getArraySize(ty) as int;

        if size == 0 {
            #fmt["/* FIXME: zero-sized array */\n"]
        } else if size == 1 {
            a_ty
        } else {
            let mut rust_ty = ~"(";
            let mut i = 1;
            while i < size {
                rust_ty += a_ty + ~",";
                i += 1;
            }
            rust_ty += a_ty + ~")";
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
        if def.kind == CXCursor_NoDeclFound ||
           def.kind == CXCursor_InvalidFile {
            opaque_decl(ctx, decl);
        }
    }
}

extern fn visit_struct(++cursor: CXCursor,
                       ++_parent: CXCursor,
                       data: CXClientData) -> c_uint unsafe {
    let ctx = *(data as *@bind_ctx);
    if cursor.kind == CXCursor_FieldDecl {
        let ty = clang_getCursorType(cursor);
        let mut name = to_str(clang_getCursorSpelling(cursor));
        if str::is_empty(name) {
            name = ~"field_unnamed" + uint::str(ctx.unnamed_field);
            ctx.unnamed_field += 1u;
        }
        ctx.out.write_line(#fmt["    %s: %s,",
                                rust_id(ctx, name),
                                conv_ty(ctx, ty, cursor, true)]);
    }
    return CXChildVisit_Continue;
}

extern fn visit_enum(++cursor: CXCursor,
                    ++parent: CXCursor,
                    data: CXClientData) -> c_uint unsafe {
    let ctx = *(data as *@bind_ctx);
    if cursor.kind == CXCursor_EnumConstantDecl {
        let int_ty =
            if clang_getEnumDeclIntegerType(parent).kind == CXType_Int {
                ~"i32"
            } else {
                ~"u32"
            };

        ctx.out.write_line(#fmt[
            "const %s: %s = %?_%s;",
            to_str(clang_getCursorSpelling(cursor)),
            int_ty,
            clang_getEnumConstantDeclValue(cursor),
            int_ty
        ]);
    }
    return CXChildVisit_Continue;
}

extern fn visit_ty_top(++cursor: CXCursor,
                      ++_parent: CXCursor,
                      data: CXClientData) -> c_uint unsafe {
    let ctx = *(data as *@bind_ctx);
    if !match_pattern(ctx, cursor) {
        return CXChildVisit_Continue;
    }

    if cursor.kind == CXCursor_StructDecl {
        do fwd_decl(ctx, cursor) || {
            ctx.unnamed_field = 0u;
            ctx.out.write_line(#fmt["type %s = {", decl_name(ctx, cursor)]);
            clang_visitChildren(cursor, visit_struct, data);
            ctx.out.write_line(~"};\n");
        }
        return CXChildVisit_Recurse;
    } else if cursor.kind == CXCursor_UnionDecl {
        do fwd_decl(ctx, cursor) || {
            ctx.out.write_line(
                #fmt["type %s = c_void /* FIXME: union type */;\n",
                     decl_name(ctx, cursor)]
            );
        }
        return CXChildVisit_Recurse;
    } else if cursor.kind == CXCursor_EnumDecl {
        do fwd_decl(ctx, cursor) || {
            ctx.out.write_line(#fmt[
                "type %s = %s;", decl_name(ctx, cursor),
                conv_ty(ctx, clang_getEnumDeclIntegerType(cursor), cursor, false)
            ]);
            clang_visitChildren(cursor, visit_enum, data);
        }
        ctx.out.write_line(~"");
        return CXChildVisit_Continue;
    } else if cursor.kind == CXCursor_FunctionDecl {
            return CXChildVisit_Continue;
    } else if cursor.kind == CXCursor_VarDecl {
        let name = to_str(clang_getCursorSpelling(cursor));
        if sym_visited(ctx, name) {
            return CXChildVisit_Continue;
        }
        ctx.out.write_line(#fmt["/* FIXME: global variable %s */\n", name]);
        return CXChildVisit_Continue;
    } else if cursor.kind == CXCursor_TypedefDecl {
        let name = to_str(clang_getCursorSpelling(cursor));
        if sym_visited(ctx, name) {
            return CXChildVisit_Continue;
        }

        let mut under_ty = clang_getTypedefDeclUnderlyingType(cursor);
        if under_ty.kind == CXType_Unexposed {
            under_ty = clang_getCanonicalType(under_ty);
        }

        ctx.out.write_line(#fmt["type %s = %s;\n",
                                rust_id(ctx, name),
                                conv_ty(ctx, under_ty, cursor, false)]);
        opaque_ty(ctx, under_ty);
        return CXChildVisit_Continue;
    } else if cursor.kind == CXCursor_FieldDecl {
        return CXChildVisit_Continue;
    }

    return CXChildVisit_Recurse;
}

extern fn visit_func_top(++cursor: CXCursor,
                        ++_parent: CXCursor,
                        data: CXClientData) -> c_uint unsafe {
    let ctx = *(data as *@bind_ctx);
    if !match_pattern(ctx, cursor) {
        return CXChildVisit_Continue;
    }

    let linkage = clang_getCursorLinkage(cursor);
    if linkage != CXLinkage_External && linkage != CXLinkage_UniqueExternal {
        return CXChildVisit_Continue;
    }

    if cursor.kind == CXCursor_FunctionDecl {
        let name = to_str(clang_getCursorSpelling(cursor));
        if sym_visited(ctx, name) {
            return CXChildVisit_Continue;
        }

        ctx.out.write_str(#fmt["fn %s(", rust_id(ctx, name)]);
        let arg_n = clang_Cursor_getNumArguments(cursor) as int;
        let mut i = 0;
        let mut unnamed = 0;
        while i < arg_n {
            if i > 0 {
                ctx.out.write_str(~", ");
            }

            let arg = clang_Cursor_getArgument(cursor, i as c_uint);
            let arg_ty = clang_getCursorType(arg);
            let mut arg_name = to_str(clang_getCursorSpelling(arg));

            arg_name = if str::is_empty(arg_name) {
                unnamed += 1;
                #fmt["arg%d", unnamed]
            } else {
                rust_id(ctx, arg_name)
            };

            ctx.out.write_str(#fmt["++%s: %s",
                                   arg_name,
                                   conv_ty(ctx, arg_ty, cursor, false)]);
            i += 1;
        }
        let ty = clang_getCursorType(cursor);
        if clang_isFunctionTypeVariadic(ty) as uint != 0u {
            ctx.out.write_str(~"/* FIXME: variadic function */");
        }
        ctx.out.write_str(~")");
        let ret_ty = clang_getCursorResultType(cursor);
        if ret_ty.kind != CXType_Void {
            ctx.out.write_str(#fmt[" -> %s",
                                   conv_ty(ctx, ret_ty, cursor, false)]);
        }
        ctx.out.write_line(~";\n");
        return CXChildVisit_Continue;
    }

    return CXChildVisit_Recurse;
}

fn main(args: ~[~str]) unsafe {
    let mut bind_args = args;
    let bin = vec::shift(bind_args);

    match parse_args(bind_args) {
        err(e) => { fail e; }
        usage => { print_usage(bin); }
        ok(clang_args, ctx) => {
            let ix = clang_createIndex(0 as c_int, 1 as c_int);
            if ix as int == 0 {
                fail ~"clang failed to create index";
            }

            let c_args = vec::map(clang_args, |s| {
                str::as_c_str(s, |b| b )
            });
            let unit = clang_parseTranslationUnit(
                ix, ptr::null(),
                vec::unsafe::to_ptr(c_args),
                vec::len(c_args) as c_int,
                ptr::null(),
                0 as c_uint, 0 as c_uint
            );
            if unit as int == 0 {
                fail ~"No input files given";
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
                return;
            }

            ctx.out.write_line(
                ~"/* automatically generated by rust-bindgen */\n"
            );

            let cursor = clang_getTranslationUnitCursor(unit);
            ctx.out.write_line(~"import libc::*;\n");

            clang_visitChildren(cursor, visit_ty_top,
                                ptr::addr_of(ctx) as CXClientData);

            ctx.out.write_line(#fmt["#[link_name=\"%s\"]", ctx.link]);
            ctx.out.write_line(~"extern mod bindgen {\n");
            clang_visitChildren(cursor, visit_func_top,
                                ptr::addr_of(ctx) as CXClientData);
            ctx.out.write_line(~"}");

            clang_disposeTranslationUnit(unit);
            clang_disposeIndex(ix);
        }
    }
}
