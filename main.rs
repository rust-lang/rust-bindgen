#![allow(non_uppercase_pattern_statics)]
#![allow(unused_must_use)]

use collections::{HashSet};
use std::{os, path};
use std::io;
use std::io::fs;

use parser;
use gen;
use types::Global;

struct BindGenOptions {
    pub match_pat: Vec<~str>,
    pub abi: ~str,
    pub builtins: bool,
    pub links: Vec<~str>,
    pub emit_ast: bool,
    pub fail_on_bitfield: bool,
    pub fail_on_unknown_type: bool,
    pub clang_args: Vec<~str>,
}

enum ParseResult {
    CmdUsage,
    ParseOk(BindGenOptions, Box<io::Writer>),
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
    let mut fail_on_unknown_type = true;

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
                "-allow-unknown-types" => {
                  fail_on_unknown_type = false;
                  ix += 1u;
                }
                _ => {
                    clang_args.push(args[ix].clone());
                    ix += 1u;
                }
            }
        }
    }

    let options = BindGenOptions {
        match_pat: pat,
        abi: abi,
        builtins: builtins,
        links: links,
        emit_ast: emit_ast,
        fail_on_bitfield: fail_on_bitfield,
        fail_on_unknown_type: fail_on_unknown_type,
        clang_args: clang_args,
    };

    return ParseOk(options, out);
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
    -allow-unknown-types  Don't fail if we encounter types we do not support,
                          instead treat them as void
    -emit-clang-ast       Output the ast (for debugging purposes)

    Options other than stated above are passed to clang.
"
    );
}

fn parse_headers(options: &BindGenOptions) -> Vec<Global> {
    let clang_opts = parser::ClangParserOptions {
        builtin_names: builtin_names(),
        builtins: options.builtins,
        match_pat: options.match_pat.clone(),
        emit_ast: options.emit_ast,
        fail_on_bitfield: options.fail_on_bitfield,
        fail_on_unknown_type: options.fail_on_unknown_type,
        clang_args: options.clang_args.clone(),
    };

    parser::parse(clang_opts).unwrap()
}

pub fn main() {
    let mut bind_args = os::args();
    let bin = bind_args.shift().unwrap();

    match parse_args(bind_args.as_slice()) {
        ParseErr(e) => fail!(e),
        CmdUsage => print_usage(bin),
        ParseOk(options, out) => {
            let globals = parse_headers(&options);
            gen::gen_rs(out, options.abi.as_slice(), options.links.as_slice(), globals);
        }
    }
}
