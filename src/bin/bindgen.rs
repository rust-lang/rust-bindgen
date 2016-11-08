#![crate_name = "bindgen"]
#![crate_type = "bin"]

extern crate bindgen;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate clang_sys;
extern crate rustc_serialize;

use bindgen::{BindgenOptions, Bindings, LinkType, clang_version};
use std::default::Default;
use std::env;
use std::fs;
use std::io;
use std::path;
use std::process;

const USAGE: &'static str = "
Usage:
    bindgen [options] \
        [--link=<lib>...] \
        [--static-link=<lib>...] \
        [--framework-link=<framework>...] \
        [--raw-line=<raw>...] \
        [--opaque-type=<type>...] \
        [--blacklist-type=<type>...] \
        [--whitelist-type=<type>...] \
        [--whitelist-function=<name>...] \
        [--whitelist-var=<name>...] \
        [--bitfield-enum=<name>...] \
        <input-header> \
        [-- <clang-args>...]

    bindgen (-h | --help)

Options:
    -h, --help                    Display this help message.

    -l=<lib>, --link=<lib>        Link to a dynamic library, can be provided
                                  multiple times.

    --static-link=<lib>           Link to a static library, can be provided
                                  multiple times.

    --framework-link=<framework>  Link to a framework.

    -o=<output-rust-file>         Write bindings to <output-rust-file>
                                  (defaults to stdout)

    --builtins                    Output bindings for builtin definitions (for
                                  example __builtin_va_list)

    --ignore-functions            Don't generate bindings for functions and
                                  methods. This is useful when you only care
                                  about struct layouts.

    --ignore-methods              Avoid generating all kind of methods.

    --enable-cxx-namespaces       Enable support for C++ namespaces.

    --emit-clang-ast              Output the ast (for debugging purposes)

    --use-msvc-mangling           Handle MSVC C++ ABI mangling; requires that
                                  target be set to (i686|x86_64)-pc-win32

    --no-convert-floats           Don't convert floats automatically to f32/f64.

    --raw-line=<raw>              Add a raw line at the beginning of the output.

    --no-unstable-rust            Avoid generating unstable rust.

    --use-core                    Use built-in types from core instead of std.

    --ctypes-prefix=<prefix>      Use the given prefix before the raw types
                                  instead of ::std::os::raw::.

    --opaque-type=<type>          Mark a type as opaque.

    --blacklist-type=<type>       Mark a type as hidden.

    --whitelist-type=<type>       Whitelist the type. If this set or any other
                                  of the whitelisting sets is not empty, then
                                  all the non-whitelisted types (or dependant)
                                  won't be generated.

    --whitelist-function=<regex>  Whitelist all the free-standing functions
                                  matching <regex>.  Same behavior on emptyness
                                  than the type whitelisting.

    --whitelist-var=<regex>       Whitelist all the free-standing variables
                                  matching <regex>.  Same behavior on emptyness
                                  than the type whitelisting.

    --bitfield-enum=<regex>       Mark any enum whose name matches <regex> as a
                                  set of bitfield flags instead of an
                                  enumeration.

    --dummy-uses=<path>           For testing purposes, generate a C/C++ file
                                  containing dummy uses of all types defined in
                                  the input header.

    <clang-args>                  Options other than stated above are passed
                                  directly through to clang.
";

// FIXME(emilio): Replace this with docopt if/when they fix their exponential
// algorithm for argument parsing.
//
// FIXME(fitzgen): Switch from `BindgenOptions` to the non-deprecated `Builder`.
#[allow(deprecated)]
fn parse_args_or_exit(args: Vec<String>) -> (BindgenOptions, Box<io::Write>) {
    let mut options = BindgenOptions::default();
    let mut dest_file = None;
    let mut source_file = None;

    let mut iter = args.into_iter().skip(1);
    loop {
        let next = match iter.next() {
            Some(arg) => arg,
            _ => break,
        };

        match &*next {
            "-h" | "--help" => {
                println!("{}", USAGE);
                process::exit(0);
            }
            "-l" | "--link" => {
                let lib = iter.next().expect("--link needs an argument");
                options.links.push((lib, LinkType::Default));
            }
            "--static-link" => {
                let lib = iter.next().expect("--static-link needs an argument");
                options.links.push((lib, LinkType::Static));
            }
            "--framework-link" => {
                let lib = iter.next()
                    .expect("--framework-link needs an argument");
                options.links.push((lib, LinkType::Framework));
            }
            "--raw-line" => {
                let line = iter.next().expect("--raw-line needs an argument");
                options.raw_lines.push(line);
            }
            "--opaque-type" => {
                let ty_canonical_name = iter.next()
                    .expect("--opaque-type expects a type");
                options.opaque_types.insert(ty_canonical_name);
            }
            "--blacklist-type" => {
                let ty_canonical_name = iter.next()
                    .expect("--blacklist-type expects a type");
                options.hidden_types.insert(ty_canonical_name);
            }
            "--whitelist-type" => {
                let ty_pat = iter.next()
                    .expect("--whitelist-type expects a type pattern");
                options.whitelisted_types.insert(&ty_pat);
            }
            "--whitelist-function" => {
                let function_pat = iter.next()
                    .expect("--whitelist-function expects a pattern");
                options.whitelisted_functions.insert(&function_pat);
            }
            "--whitelist-var" => {
                let var_pat = iter.next()
                    .expect("--whitelist-var expects a pattern");
                options.whitelisted_vars.insert(&var_pat);
            }
            "--bitfield-enum" => {
                let enum_pat = iter.next()
                    .expect("--bitfield-enum expects a pattern");
                options.bitfield_enums.insert(&enum_pat);
            }
            "--" => {
                while let Some(clang_arg) = iter.next() {
                    options.clang_args.push(clang_arg);
                }
            }
            "--output" | "-o" => {
                let out_name = iter.next().expect("-o expects a file name");
                dest_file = Some(out_name);
            }
            "--builtins" => {
                options.builtins = true;
            }
            "--ignore-functions" => {
                options.ignore_functions = true;
            }
            "--ignore-methods" => {
                options.ignore_methods = true;
            }
            "--enable-cxx-namespaces" => {
                options.enable_cxx_namespaces = true;
            }
            "--no-unstable-rust" => {
                options.unstable_rust = false;
            }
            "--use-core" => {
                options.use_core = true;
            }
            "--ctypes-prefix" => {
                let prefix = iter.next()
                    .expect("--ctypes-prefix expects a prefix after it");
                options.ctypes_prefix = Some(prefix);
            }
            "--emit-clang-ast" => {
                options.emit_ast = true;
            }
            "--no-convert-floats" => {
                options.convert_floats = false;
            }
            "--use-msvc-mangling" => {
                options.msvc_mangling = true;
            }
            "--dummy-uses" => {
                let dummy_path = iter.next()
                    .expect("--dummy-uses expects a file path");
                options.dummy_uses = Some(dummy_path);
            }
            other if source_file.is_none() => {
                source_file = Some(other.into());
            }
            other => {
                panic!("Unknown option: \"{}\"", other);
            }
        }
    }

    if let Some(source_file) = source_file.take() {
        options.clang_args.push(source_file);
        options.input_header = options.clang_args.last().cloned();
    } else {
        options.input_header = options.clang_args
            .iter()
            .find(|arg| arg.ends_with(".h") || arg.ends_with(".hpp"))
            .cloned();
    }

    let out = if let Some(ref path_name) = dest_file {
        let path = path::Path::new(path_name);
        let file = fs::File::create(path).expect("Opening out file failed");
        Box::new(io::BufWriter::new(file)) as Box<io::Write>
    } else {
        Box::new(io::BufWriter::new(io::stdout())) as Box<io::Write>
    };

    (options, out)
}

pub fn main() {
    log::set_logger(|max_log_level| {
            use env_logger::Logger;
            let env_logger = Logger::new();
            max_log_level.set(env_logger.filter());
            Box::new(env_logger)
        })
        .expect("Failed to set logger.");

    let mut bind_args: Vec<_> = env::args().collect();

    let version = clang_version();
    let expected_version = if cfg!(feature = "llvm_stable") {
        (3, 8)
    } else {
        (3, 9)
    };

    info!("Clang Version: {}", version.full);

    match version.parsed {
        None => warn!("Couldn't parse libclang version"),
        Some(version) if version != expected_version => {
            error!("Using clang {:?}, expected {:?}",
                   version,
                   expected_version);
        }
        _ => {}
    }

    if let Some(clang) = clang_sys::support::Clang::find(None) {
        let has_clang_args =
            bind_args.iter().rposition(|arg| *arg == "--").is_some();
        if !has_clang_args {
            bind_args.push("--".to_owned());
        }

        // If --target is specified, assume caller knows what they're doing and
        // don't mess with
        // include paths for them
        let has_target_arg = bind_args.iter()
            .rposition(|arg| arg.starts_with("--target"))
            .is_some();
        if !has_target_arg {
            // TODO: distinguish C and C++ paths? C++'s should be enough, I
            // guess.
            for path in clang.cpp_search_paths.into_iter() {
                if let Ok(path) = path.into_os_string().into_string() {
                    bind_args.push("-isystem".to_owned());
                    bind_args.push(path);
                }
            }
        }
    }

    let (options, out) = parse_args_or_exit(bind_args);

    let mut bindings = Bindings::generate(options, None)
        .expect("Unable to generate bindings");

    bindings.write_dummy_uses()
        .expect("Unable to write dummy uses to file.");

    bindings.write(out)
        .expect("Unable to write bindings to file.");
}
