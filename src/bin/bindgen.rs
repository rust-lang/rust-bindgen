#![crate_name = "bindgen"]
#![crate_type = "bin"]

extern crate bindgen;
extern crate docopt;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate clang_sys;
extern crate rustc_serialize;

use bindgen::clang_version;
use docopt::Docopt;
use std::env;
use std::fs;
use std::io;
use std::path;

const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

const USAGE: &'static str = "
Usage:
    bindgen [options] <input-header> [-- <clang-args>...]
    bindgen (--help | --version)

Options:
    --help, -h                    Display this help message.

    --version, -V                 Display version information.

    --link=<lib>, -l <lib> ...    Link to a dynamic library. Can be provided
                                  multiple times.

    --static-link=<lib> ...       Link to a static library. Can be provided
                                  multiple times.

    --framework-link=<lib> ...    Link to a framework. Can be provided
                                  multiple times.

    --output=<file>, -o <file>    Write Rust bindings to <file>.

    --builtins                    Output bindings for builtin definitions (for
                                  example __builtin_va_list)

    --ignore-functions            Don't generate bindings for functions and
                                  methods. This is useful when you only care
                                  about struct layouts.

    --ignore-methods              Avoid generating all kinds of methods.

    --enable-cxx-namespaces       Enable support for C++ namespaces.

    --emit-clang-ast              Output the Clang AST for debugging purposes.

    --raw-line=<raw> ...          Add a raw line at the beginning of the output.

    --no-unstable-rust            Avoid generating unstable Rust code.

    --opaque-type=<type> ...      Mark a type as opaque.

    --blacklist-type=<type> ...   Mark a type as hidden.

    --whitelist-type=<type> ...   Whitelist the type. If this set or any other
                                  of the whitelisting sets is not empty, then
                                  all the non-whitelisted types (or anything
                                  that depends on them) won't be generated.

    --whitelist-function=<regex> ...  Whitelist all the free-standing functions
                                  matching <regex>. Same behavior on emptyness
                                  as whitelisted types.

    --whitelist-var=<regex> ...   Whitelist all the free-standing variables
                                  matching <regex>. Same behavior on emptyness
                                  as whitelisted types.

    --dummy-uses=<path>           For testing purposes, generate a C/C++ file
                                  containing dummy uses of all types defined in
                                  the input header.

    <clang-args>                  Pass arguments through to clang.

Deprecated:

    --use-msvc-mangling           Handle MSVC C++ ABI mangling; requires that
                                  target be set to (i686|x86_64)-pc-win32.
";

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

    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());

    if args.get_bool("--version") {
        println!("{} v{}", PKG_NAME, PKG_VERSION);
        std::process::exit(0);
    }

    let mut builder = bindgen::builder();

    // Input header
    let header = args.get_str("<input-header>");
    if !header.is_empty() {
        builder = builder.header(header);
    }

    // Output file or stdout
    let output = args.get_str("--output");
    let out = if !output.is_empty() {
        let path = path::Path::new(output);
        let file = fs::File::create(path).expect("Opening output file failed.");
        Box::new(io::BufWriter::new(file)) as Box<io::Write>
    } else {
        Box::new(io::BufWriter::new(io::stdout())) as Box<io::Write>
    };

    // Emit C/C++ type uses file for testing
    let dummy_uses = args.get_str("--dummy-uses");
    if !dummy_uses.is_empty() {
        builder = builder.dummy_uses(dummy_uses);
    }

    if args.get_bool("--builtins") {
        builder = builder.emit_builtins();
    }

    if args.get_bool("--emit-clang-ast") {
        builder = builder.emit_clang_ast();
    }

    if args.get_bool("--enable-cxx-namespaces") {
        builder = builder.enable_cxx_namespaces();
    }

    if args.get_bool("--ignore-functions") {
        builder = builder.ignore_functions();
    }

    if args.get_bool("--ignore-methods") {
        builder = builder.ignore_methods();
    }

    // Do not generate unstable Rust code
    if args.get_bool("--no-unstable-rust") {
        builder = builder.no_unstable_rust();
    }

    // Shared library links
    for link in args.get_vec("--link") {
        builder = builder.link(link);
    }

    // Static library links
    for link in args.get_vec("--static-link") {
        builder = builder.link_static(link);
    }

    // Framework links
    for link in args.get_vec("--framework-link") {
        builder = builder.link_framework(link);
    }

    // Raw lines to add at top of output
    for line in args.get_vec("--raw-line") {
        builder = builder.raw_line(line);
    }

    // Hidden types
    for arg in args.get_vec("--blacklist-type") {
        builder = builder.hide_type(arg);
    }

    // Opaque types
    for arg in args.get_vec("--opaque-type") {
        builder = builder.opaque_type(arg);
    }

    // Whitelisted types
    for regex in args.get_vec("--whitelist-type") {
        builder = builder.whitelisted_type(regex);
    }

    // Whitelisted functions
    for regex in args.get_vec("--whitelist-function") {
        builder = builder.whitelisted_function(regex);
    }

    // Whitelisted variables
    for regex in args.get_vec("--whitelist-var") {
        builder = builder.whitelisted_var(regex);
    }

    // Clang parameters
    for arg in args.get_vec("<clang-args>") {
        builder = builder.clang_arg(arg);
    }

    let mut bindings = builder.generate()
        .expect("Unable to generate bindings");

    bindings.write_dummy_uses()
        .expect("Unable to write dummy uses to file.");

    bindings.write(out)
        .expect("Unable to write bindings to file.");
}
