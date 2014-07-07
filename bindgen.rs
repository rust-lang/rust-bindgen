#![crate_name = "bindgen"]
#![crate_type = "bin"]
#![feature(phase)]

extern crate bindgen;
#[phase(plugin, link)] extern crate log;
extern crate syntax;

use bindgen::{Logger, generate_bindings, BindgenOptions};
use std::{io, os, path};
use std::default::Default;
use std::io::{fs, IoResult};
use syntax::ast;
use syntax::codemap::DUMMY_SP;
use syntax::print::pprust;
use syntax::print::pp::eof;

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
    ParseOk(BindgenOptions, Box<io::Writer>),
    ParseErr(String)
}

fn parse_args(args: &[String]) -> ParseResult {
    let args_len = args.len();

    let mut options: BindgenOptions = Default::default();
    let mut out = box io::BufferedWriter::new(io::stdout()) as Box<io::Writer>;

    if args_len == 0u {
        return CmdUsage;
    }

    let mut ix = 0u;
    while ix < args_len {
        if args[ix].len() > 2 && args[ix].as_slice().slice_to(2) == "-l" {
            options.links.push((args[ix].as_slice().slice_from(2).to_string(), None));
            ix += 1u;
        } else {
            match args[ix].as_slice() {
                "--help" | "-h" => {
                    return CmdUsage;
                }
                "-emit-clang-ast" => {
                    options.emit_ast = true;
                    ix += 1u;
                }
                "-o" => {
                    if ix + 1u >= args_len {
                        return ParseErr("Missing output filename".to_string());
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
                        return ParseErr("Missing link name".to_string());
                    }
                    options.links.push((args[ix + 1u].clone(), None));
                    ix += 2u;
                }
                "-static-link" => {
                    if ix + 1u >= args_len {
                        return ParseErr("Missing link name".to_string());
                    }
                    options.links.push((args[ix + 1u].clone(), Some("static".to_string())));
                    ix += 2u;
                }
                "-framework-link" => {
                    if ix + 1u >= args_len {
                        return ParseErr("Missing link name".to_string());
                    }
                    options.links.push((args[ix + 1u].clone(), Some("framework".to_string())));
                    ix += 2u;
                }
                "-match" => {
                    if ix + 1u >= args_len {
                        return ParseErr("Missing match pattern".to_string());
                    }
                    options.match_pat.push(args[ix + 1u].clone());
                    ix += 2u;
                }
                "-builtins" => {
                    options.builtins = true;
                    ix += 1u;
                }
                "-abi" => {
                    options.abi = args[ix + 1u].clone();
                    ix += 2u;
                }
                "-allow-bitfields" => {
                    options.fail_on_bitfield = false;
                    ix += 1u;
                }
                "-allow-unknown-types" => {
                    options.fail_on_unknown_type = false;
                    ix += 1u;
                }
                _ => {
                    options.clang_args.push(args[ix].clone());
                    ix += 1u;
                }
            }
        }
    }

    return ParseOk(options, out);
}

fn print_usage(bin: String) {
    io::stdio::print(format!("Usage: {} [options] input.h", bin.as_slice()).append(
"
Options:
    -h or --help           Display help message
    -l <name> or -l<name>  Link to a dynamic library, can be proivded
                           multiple times
    -static-link <name>    Link to a static library
    -framework-link <name> Link to a framework
    -o <output.rs>         Write bindings to <output.rs> (default stdout)
    -match <name>          Only output bindings for definitions from files
                           whose name contains <name>
                           If multiple -match options are provided, files
                           matching any rule are bound to.
    -builtins              Output bindings for builtin definitions
                           (for example __builtin_va_list)
    -abi <abi>             Indicate abi of extern functions (default C)
    -allow-bitfields       Don't fail if we encounter a bitfield
                           (note that bindgen does not support bitfields)
    -allow-unknown-types   Don't fail if we encounter types we do not support,
                           instead treat them as void
    -emit-clang-ast        Output the ast (for debugging purposes)

    Options other than stated above are passed to clang.
"
    ).as_slice());
}

fn try_pprint(module: &ast::Mod, out: Box<io::Writer>) -> IoResult<()> {
    let mut ps = pprust::rust_printer(out);
    try!(ps.s.out.write("/* automatically generated by rust-bindgen */\n\n".as_bytes()));

    try!(ps.print_mod(module, &[]));
    try!(ps.print_remaining_comments());
    try!(eof(&mut ps.s));

    ps.s.out.flush()
}

#[main]
pub fn main() {
    let mut bind_args = os::args();
    let bin = bind_args.shift().unwrap();

    match parse_args(bind_args.as_slice()) {
        ParseErr(e) => fail!(e),
        CmdUsage => print_usage(bin),
        ParseOk(options, out) => {
            let logger = StdLogger;
            match generate_bindings(options, Some(&logger as &Logger), DUMMY_SP) {
                Ok(items) => {
                    let module = ast::Mod {
                        inner: DUMMY_SP,
                        view_items: Vec::new(),
                        items: items,
                    };

                    match try_pprint(&module, out) {
                        Err(e) => logger.error(format!("Unable to write bindings to file. {}", e).as_slice()),
                        _ => ()
                    }
                }
                Err(_) => ()
            }
        }
    }
}
