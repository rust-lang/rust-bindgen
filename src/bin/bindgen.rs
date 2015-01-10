#![crate_name = "bindgen"]
#![crate_type = "bin"]

extern crate bindgen;
#[macro_use] extern crate log;
extern crate syntax;

use bindgen::{Bindings, BindgenOptions, LinkType, Logger};
use std::{io, os, path};
use std::default::Default;
use std::io::fs;

struct StdLogger;

impl Logger for StdLogger {
    fn error(&self, msg: &str) {
        error!("{}", msg);
    }

    fn warn(&self, msg: &str) {
        warn!("{}", msg);
    }
}

enum ParseResult {
    CmdUsage,
    ParseOk(BindgenOptions, Box<io::Writer+'static>),
    ParseErr(String)
}

fn parse_args(args: &[String]) -> ParseResult {
    let args_len = args.len();

    let mut options: BindgenOptions = Default::default();
    let mut out = Box::new(io::BufferedWriter::new(io::stdout())) as Box<io::Writer>;

    if args_len == 0 {
        return ParseResult::CmdUsage;
    }

    let mut ix = 0us;
    while ix < args_len {
        if args[ix].len() > 2 && args[ix].as_slice().slice_to(2) == "-l" {
            options.links.push((args[ix].as_slice().slice_from(2).to_string(), LinkType::Default));
            ix += 1;
        } else {
            match args[ix].as_slice() {
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
                    let path = path::Path::new(args[ix + 1].clone());
                    match fs::File::create(&path) {
                        Ok(f) => { out = Box::new(io::BufferedWriter::new(f)) as Box<io::Writer>; }
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
                "-builtins" => {
                    options.builtins = true;
                    ix += 1;
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
    let mut s = format!("Usage: {} [options] input.h", bin.as_slice());
    s.push_str(
"
Options:
    -h or --help               Display help message
    -l <name> or -l<name>      Link to a dynamic library, can be proivded
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
    io::stdio::print(s.as_slice());
}

#[main]
pub fn main() {
    let mut bind_args = os::args();
    let bin = bind_args.remove(0);

    match parse_args(bind_args.as_slice()) {
        ParseResult::ParseErr(e) => panic!(e),
        ParseResult::CmdUsage => print_usage(bin),
        ParseResult::ParseOk(options, mut out) => {
            let logger = StdLogger;
            match Bindings::generate(&options, Some(&logger as &Logger), None) {
                Ok(bindings) => match bindings.write(&mut out) {
                    Ok(()) => (),
                    Err(e) => logger.error(format!("Unable to write bindings to file. {}", e).as_slice())
                },
                Err(()) => ()
            }
        }
    }
}
