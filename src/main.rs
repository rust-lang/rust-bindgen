extern crate bindgen;
#[macro_use] extern crate log;

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
        error!("{}", msg);
    }

    fn warn(&self, msg: &str) {
        warn!("{}", msg);
    }
}

enum ParseResult {
    CmdUsage,
    ParseOk(BindgenOptions, Box<io::Write+'static>),
    ParseErr(String)
}

fn parse_args(args: &[String]) -> ParseResult {
    let args_len = args.len();

    let mut options: BindgenOptions = Default::default();
    options.derive_debug = false;
    let mut out = Box::new(io::BufWriter::new(io::stdout())) as Box<io::Write>;

    if args_len == 0 {
        return ParseResult::CmdUsage;
    }

    let mut ix: usize = 0;
    while ix < args_len {
        if args[ix].len() > 2 && &args[ix][..2] == "-l" {
            options.links.push((args[ix][2..].to_string(), LinkType::Dynamic));
            ix += 1;
        } else {
            match &args[ix][..] {
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
                    let parts = args[ix + 1].split('=').collect::<Vec<_>>();
                    options.links.push(match parts.len() {
                        1 => (parts[0].to_string(), LinkType::Dynamic),
                        2 => (parts[1].to_string(), match parts[0] {
                            "static" => LinkType::Static,
                            "dynamic" => LinkType::Dynamic,
                            "framework" => LinkType::Framework,
                            _ => return ParseResult::ParseErr("Invalid link kind".to_string()),
                        }),
                        _ => return ParseResult::ParseErr("Invalid link name".to_string()),
                    });
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
                "-no-rust-enums" => {
                    options.rust_enums = false;
                    ix += 1;
                }
                "-derive-debug" => {
                    options.derive_debug = true;
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
    let mut s = format!("Usage: {} [OPTIONS] HEADERS...", &bin[..]);
    s.push_str(
"

Options:
    -h, --help                  Display help message
    -l [KIND=]NAME              Link to the specified library NAME. The optional KIND can be one of,
                                static, dylib, or framework. If omitted, dylib is assumed.
    -o FILENAME                 Write generated bindings to FILENAME (default is stdout)
    -match NAME                 Only output bindings for definitions from files whose names contain
                                NAME. Can be used multiples times to include files matching any of
                                the names.
    -builtins                   Output bindings for builtin definitions (for example,
                                `__builtin_va_list`)
    -allow-unknown-types        Do not fail if unknown types are encountered; instead treat them as
                                `void`
    -emit-clang-ast             Output the AST (for debugging purposes)
    -override-enum-type TYPE    Override the integer type for enums, where TYPE is one of:
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
    
    Options other than the above are passed to Clang.
"
    );
    print!("{}", &s[..]);
}

pub fn main() {
    let mut bind_args: Vec<_> = env::args().collect();
    let bin = bind_args.remove(0);

    match parse_args(&bind_args[..]) {
        ParseResult::ParseErr(e) => panic!(e),
        ParseResult::CmdUsage => print_usage(bin),
        ParseResult::ParseOk(options, out) => {
            let logger = StdLogger;
            match Bindings::generate(&options, Some(&logger as &Logger), None) {
                Ok(bindings) => match bindings.write(out) {
                    Ok(()) => (),
                    Err(e) => {
                        logger.error(&format!("Unable to write bindings to file. {}", e)[..]);
                        exit(-1);
                    }
                },
                Err(()) => exit(-1)
            }
        }
    }
}
