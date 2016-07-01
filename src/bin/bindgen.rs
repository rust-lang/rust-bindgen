#![crate_name = "bindgen"]
#![crate_type = "bin"]

extern crate bindgen;
#[macro_use]
extern crate log;

use bindgen::{Bindings, BindgenOptions, LinkType, Logger};
use std::io;
use std::path;
use std::env;
use std::default::Default;
use std::fs;
use std::process::exit;

struct StdLogger;

impl Logger for StdLogger {
    fn error(&self, msg: &str) {
        println!("{}", msg);
    }

    fn warn(&self, msg: &str) {
        println!("{}", msg);
    }
}

enum ParseResult {
    CmdUsage,
    ParseOk(BindgenOptions, Box<io::Write>),
    ParseErr(String)
}

fn parse_args(args: &[String]) -> ParseResult {
    let args_len = args.len();

    let mut options: BindgenOptions = Default::default();
    let mut out = Box::new(io::BufWriter::new(io::stdout())) as Box<io::Write>;

    if args_len == 0 {
        return ParseResult::CmdUsage;
    }

    let mut ix: usize = 0;
    while ix < args_len {
        if args[ix].len() > 2 && &args[ix][..2] == "-l" {
            options.links.push((args[ix][2..].to_string(), LinkType::Default));
            ix += 1;
        } else {
            match &*args[ix] {
                "--help" | "-h" => {
                    return ParseResult::CmdUsage;
                }
                "-emit-clang-ast" => {
                    options.emit_ast = true;
                    ix += 1;
                }
                "-o" => {
                    if ix + 1 >= args_len {
                        return ParseResult::ParseErr("Missing output filename".to_string());
                    }
                    let path = path::Path::new(&args[ix + 1]);
                    match fs::File::create(&path) {
                        Ok(f) => { out = Box::new(io::BufWriter::new(f)) as Box<io::Write>; }
                        Err(_) => { return ParseResult::ParseErr(format!("Open {} failed", args[ix + 1])); }
                    }
                    ix += 2;
                }
                "-l" => {
                    if ix + 1 >= args_len {
                        return ParseResult::ParseErr("Missing link name".to_string());
                    }
                    options.links.push((args[ix + 1].clone(), LinkType::Default));
                    ix += 2;
                }
                "-static-link" => {
                    if ix + 1 >= args_len {
                        return ParseResult::ParseErr("Missing link name".to_string());
                    }
                    options.links.push((args[ix + 1].clone(), LinkType::Static));
                    ix += 2;
                }
                "-framework-link" => {
                    if ix + 1 >= args_len {
                        return ParseResult::ParseErr("Missing link name".to_string());
                    }
                    options.links.push((args[ix + 1].clone(), LinkType::Framework));
                    ix += 2;
                }
                "-match" => {
                    if ix + 1 >= args_len {
                        return ParseResult::ParseErr("Missing match pattern".to_string());
                    }
                    options.match_pat.push(args[ix + 1].clone());
                    ix += 2;
                }
                "-blacklist-type" => {
                    if ix + 1 >= args_len {
                        return ParseResult::ParseErr("Missing blacklist type name".to_string());
                    }
                    options.blacklist_type.push(args[ix + 1].clone());
                    ix += 2;
                }
                "-opaque-type" => {
                    if ix + 1 >= args_len {
                        return ParseResult::ParseErr("Missing opaque type name".to_string());
                    }
                    options.opaque_types.push(args[ix + 1].clone());
                    ix += 2;
                }
                "-builtins" => {
                    options.builtins = true;
                    ix += 1;
                }
                "-ignore-functions" => {
                    options.ignore_functions = true;
                    ix += 1;
                }
                "-no-bitfield-methods" => {
                    options.gen_bitfield_methods = false;
                    ix += 1;
                }
                "-no-class-constants" => {
                    options.class_constants = false;
                    ix += 1;
                }
                "-dtor-attr" => {
                    if ix + 1 >= args_len {
                        return ParseResult::ParseErr("Missing dtor attr".to_string());
                    }
                    options.dtor_attrs.push(args[ix + 1].clone());
                    ix += 2;
                }
                "-allow-unknown-types" => {
                    options.fail_on_unknown_type = false;
                    ix += 1;
                }
                "-override-enum-type" => {
                    if ix + 1 >= args_len {
                        return ParseResult::ParseErr("Missing enum type".to_string());
                    }
                    options.override_enum_ty = args[ix + 1].clone();
                    ix += 2;
                }
                "-enable-cxx-namespaces" => {
                    options.enable_cxx_namespaces = true;
                    ix += 1;
                }
                "-no-type-renaming" => {
                    options.rename_types = false;
                    ix += 1;
                }
                "-raw-line" => {
                    if ix + 1 >= args_len {
                        return ParseResult::ParseErr("Missing raw-line argument".to_string());
                    }
                    options.raw_lines.push(args[ix + 1].clone());
                    ix += 2;
                }
                _ => {
                    options.clang_args.push(args[ix].clone());
                    ix += 1;
                }
            }
        }
    }

    return ParseResult::ParseOk(options, out);
}

fn print_usage(bin: String) {
    let mut s = format!("Usage: {} [options] input.h", &bin);
    s.push_str(
"
Options:
    -h or --help               Display help message
    -l <name> or -l<name>      Link to a dynamic library, can be provided
                               multiple times
    -static-link <name>        Link to a static library
    -framework-link <name>     Link to a framework
    -o <output.rs>             Write bindings to <output.rs> (default stdout)
    -match <name>              Only output bindings for definitions from files
                               whose name contains <name>
                               If multiple -match options are provided, files
                               matching any rule are bound to.
    -builtins                  Output bindings for builtin definitions
                               (for example __builtin_va_list)
    -ignore-functions          Don't generate bindings for functions and methods.
                               This is useful when you only care about struct layouts.
    -enable-cxx-namespaces     Enable support for C++ namespaces.
    -no-type-renaming          Don't rename types.
    -allow-unknown-types       Don't fail if we encounter types we do not support,
                               instead treat them as void
    -emit-clang-ast            Output the ast (for debugging purposes)
    -override-enum-type <type> Override enum type, type name could be
                                 uchar
                                 schar
                                 ushort
                                 sshort
                                 uint
                                 sint
                                 ulong
                                 slong
                                 ulonglong
                                 slonglong

    Options other than stated above are passed to clang.
"
    );
    println!("{}", &s);
}

pub fn main() {
    let mut bind_args: Vec<_> = env::args().collect();
    let bin = bind_args.remove(0);

    match bindgen::get_include_dir() {
        Some(path) => {
            bind_args.push("-I".to_owned());
            bind_args.push(path);
        }
        None => (),
    }

    match parse_args(&bind_args) {
        ParseResult::ParseErr(e) => panic!(e),
        ParseResult::CmdUsage => print_usage(bin),
        ParseResult::ParseOk(options, out) => {
            let logger = StdLogger;
            match Bindings::generate(&options, Some(&logger as &Logger), None) {
                Ok(bindings) => match bindings.write(out) {
                    Ok(()) => (),
                    Err(e) => {
                        logger.error(&format!("Unable to write bindings to file. {}", e));
                        exit(-1);
                    }
                },
                Err(()) => exit(-1)
            }
        }
    }
}
