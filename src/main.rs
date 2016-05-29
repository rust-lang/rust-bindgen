extern crate bindgen;
#[macro_use]
extern crate log;
extern crate docopt;
#[macro_use]
extern crate rustc_serialize;
extern crate env_logger;

use bindgen::{Builder, LinkType};
use std::io::{self, Write};
use std::fs::File;
use std::process::exit;

const USAGE: &'static str = "
Generate C bindings for Rust.

Usage:
  bindgen [options] <file> [-- <clang-args>...]
  bindgen (-h | --help)

Options:
  <clang-args>                 Options passed directly to clang.
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
  --use-core                  Use `core` as a base crate for `Option` and such.
                              See also `--ctypes-prefix`.
  --ctypes-prefix=<prefix>    Use this prefix for all the types in the generated
                              code.
                              [default: std::os::raw]
  --remove-prefix=<prefix>    Prefix to remove from all the symbols, like
                              `libfoo_`. The removal is case-insensitive.
  --no-derive-debug           Disable `derive(Debug)` for all generated types.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_file: String,
    arg_clang_args: Vec<String>,
    flag_link: Option<String>,
    flag_output: String,
    flag_match: Option<String>,
    flag_builtins: bool,
    flag_emit_clang_ast: bool,
    flag_override_enum_type: String,
    flag_ctypes_prefix: String,
    flag_use_core: bool,
    flag_remove_prefix: Option<String>,
    // TODO: allow finer control.
    flag_no_derive_debug: bool,
}

fn args_to_opts(args: Args, builder: &mut Builder) {
    builder.header(args.arg_file)
           .emit_ast(args.flag_emit_clang_ast)
           .ctypes_prefix(args.flag_ctypes_prefix
                              .split("::")
                              .map(String::from)
                              .collect::<Vec<_>>())
           .use_core(args.flag_use_core)
           .derive_debug(!args.flag_no_derive_debug)
           .override_enum_ty(args.flag_override_enum_type);
    for arg in args.arg_clang_args {
        builder.clang_arg(arg);
    }
    if let Some(s) = args.flag_match {
        builder.match_pat(s);
    }
    if let Some(s) = args.flag_remove_prefix {
        builder.remove_prefix(s);
    }
    if args.flag_builtins {
        builder.builtins();
    }
    if let Some(link) = args.flag_link {
        let mut parts = link.split('=');
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
                println!("Wrong link format: {}", link);
                exit(1);
            }
        };
        builder.link(lib, kind);
    }
}

fn get_output(o: &str) -> Box<Write> {
    if o == "-" {
        Box::new(io::stdout())
    } else {
        Box::new(File::create(o).expect(&format!("\"{}\" unwritable", o)))
    }
}

pub fn main() {
    env_logger::LogBuilder::new().parse("bindgen=warn").init().unwrap();

    let args: Args = docopt::Docopt::new(USAGE)
                         .and_then(|d| d.decode())
                         .unwrap_or_else(|e| e.exit());
    debug!("{:?}", args);

    let output = get_output(&args.flag_output);

    let mut builder = Builder::new();
    args_to_opts(args, &mut builder);
    debug!("{:?}", builder);

    match builder.generate() {
        Ok(bindings) => {
            match bindings.write(output) {
                Ok(()) => (),
                Err(e) => {
                    error!("Unable to write bindings to file. {}", e);
                    exit(-1);
                }
            }
        }
        Err(()) => exit(-1),
    }
}
