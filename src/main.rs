extern crate bindgen;
#[macro_use]
extern crate log;
extern crate docopt;
#[macro_use]
extern crate rustc_serialize;
extern crate env_logger;

use bindgen::{Builder, LinkType, Logger};
use std::io::{self, Write};
use std::fs::File;
use std::process::exit;

#[derive(Debug)]
struct StdLogger;

impl Logger for StdLogger {
    fn error(&self, msg: &str) {
        error!("{}", msg);
    }

    fn warn(&self, msg: &str) {
        warn!("{}", msg);
    }
}

const USAGE: &'static str = "
Generate C bindings for Rust.

Usage:
  bindgen [options] <file>
  bindgen (-h | --help)

Options:
  -h, --help                   Display this help message.
  --link=<library>             Link to a dynamic library, can be provided multiple times.
                               <library> is in the format `[kind=]lib`, where `kind` is
                               one of `static`, `dynamic` or `framework`.
  --output=<output>            Write bindings to <output> (- is stdout).
                               [default: -]
  --match=<name>               Only output bindings for definitions from files
                               whose name contains <name>
                               If multiple -match options are provided, files
                               matching any rule are bound to.
  --builtins                   Output bindings for builtin definitions
                               (for example __builtin_va_list)
  --emit-clang-ast             Output the ast (for debugging purposes)
  --override-enum-type=<type>  Override enum type, type name could be
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
  --ctypes-prefix=<prefix>    Use this prefix for all the types in the generated
                              code.
                              [default: std::os::raw]
  --clang-options=<opts>      Options to clang.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_file: String,
    flag_link: String,
    flag_output: String,
    flag_match: Option<String>,
    flag_builtins: bool,
    flag_emit_clang_ast: bool,
    flag_override_enum_type: String,
    flag_clang_options: String,
    flag_ctypes_prefix: String,
}

fn args_to_opts(args: Args, builder: &mut Builder) {
    builder.header(args.arg_file)
           .emit_ast(args.flag_emit_clang_ast)
           .ctypes_prefix(args.flag_ctypes_prefix
                              .split("::")
                              .map(String::from)
                              .collect::<Vec<_>>())
           .override_enum_ty(args.flag_override_enum_type);
    for arg in args.flag_clang_options.split(" ") {
        builder.clang_arg(arg);
    }
    if let Some(s) = args.flag_match {
        builder.match_pat(s);
    }
    if args.flag_builtins {
        builder.builtins();
    }
    let mut parts = args.flag_link.split('=');
    let (lib, kind) = match (parts.next(), parts.next()) {
        (Some(lib), None) => (lib, LinkType::Dynamic),
        (Some(kind), Some(lib)) => {
            (lib,
             match kind {
                "static" => LinkType::Static,
                "dynamic" => LinkType::Dynamic,
                "framework" => LinkType::Framework,
                _ => {
                    println!("Link type unknown: {}", kind);
                    exit(1);
                }
            })
        }
        _ => {
            println!("Wrong link format: {}", args.flag_link);
            exit(1);
        }
    };
    builder.link(lib, kind);
}

fn get_output(o: &str) -> Box<Write> {
    if o == "-" {
        Box::new(io::stdout())
    } else {
        Box::new(File::open(o).expect(&format!("\"{}\" unreadable", o)))
    }
}

pub fn main() {
    env_logger::init().unwrap();

    let args: Args = docopt::Docopt::new(USAGE)
                         .and_then(|d| d.decode())
                         .unwrap_or_else(|e| e.exit());
    debug!("{:?}", args);

    let output = get_output(&args.flag_output);

    let logger = StdLogger;
    let mut builder = Builder::default();
    builder.log(&logger);
    args_to_opts(args, &mut builder);
    debug!("{:?}", builder);

    match builder.generate() {
        Ok(bindings) => {
            match bindings.write(output) {
                Ok(()) => (),
                Err(e) => {
                    logger.error(&format!("Unable to write bindings to file. {}", e)[..]);
                    exit(-1);
                }
            }
        }
        Err(()) => exit(-1),
    }
}
