#![crate_name = "bindgen"]
#![crate_type = "bin"]

extern crate bindgen;
#[macro_use]
extern crate docopt;
#[macro_use]
extern crate log;
extern crate clang_sys;
extern crate rustc_serialize;

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

const USAGE: &'static str = "
Usage:
    bindgen [options] \
        [--link=<lib>...] \
        [--static-link=<lib>...] \
        [--framework-link=<framework>...] \
        [--match=<name>...] \
        [--raw-line=<raw>...] \
        [--dtor-attr=<attr>...] \
        [--opaque-type=<type>...] \
        [--blacklist-type=<type>...] \
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

    --match=<name>                Only output bindings for definitions from
                                  files whose name contains <name>. If multiple
                                  match options are provided, files matching any
                                  rule are bound to.

    --builtins                    Output bindings for builtin definitions (for
                                  example __builtin_va_list)

    --ignore-functions            Don't generate bindings for functions and
                                  methods. This is useful when you only care
                                  about struct layouts.

    --enable-cxx-namespaces       Enable support for C++ namespaces.

    --no-type-renaming            Don't rename types.

    --allow-unknown-types         Don't fail if we encounter types we do not
                                  support, instead treat them as void

    --emit-clang-ast              Output the ast (for debugging purposes)

    --use-msvc-mangling           Handle MSVC C++ ABI mangling; requires that
                                  target be set to (i686|x86_64)-pc-win32

    --override-enum-type=<type>   Override enum type, type name could be
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

    --raw-line=<raw>              TODO
    --dtor-attr=<attr>            TODO
    --no-class-constants          TODO
    --no-unstable-rust            TODO
    --no-namespaced-constants     TODO
    --no-bitfield-methods         TODO
    --ignore-methods              TODO
    --opaque-type=<type>          TODO
    --blacklist-type=<type>       TODO

    <clang-args>                  Options other than stated above are passed
                                  directly through to clang.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_input_header: String,
    flag_link: Vec<String>,
    flag_static_link: Vec<String>,
    flag_framework_link: Vec<String>,
    flag_o: Option<String>,
    flag_match: Vec<String>,
    flag_builtins: bool,
    flag_ignore_functions: bool,
    flag_enable_cxx_namespaces: bool,
    flag_no_type_renaming: bool,
    flag_allow_unknown_types: bool,
    flag_emit_clang_ast: bool,
    flag_use_msvc_mangling: bool,
    flag_override_enum_type: String,
    flag_raw_line: Vec<String>,
    flag_dtor_attr: Vec<String>,
    flag_no_class_constants: bool,
    flag_no_unstable_rust: bool,
    flag_no_namespaced_constants: bool,
    flag_no_bitfield_methods: bool,
    flag_ignore_methods: bool,
    flag_opaque_type: Vec<String>,
    flag_blacklist_type: Vec<String>,
    arg_clang_args: Vec<String>,
}

type ParseResult<T> = Result<T, String>;

impl Into<ParseResult<(BindgenOptions, Box<io::Write>)>> for Args {
    fn into(mut self) -> Result<(BindgenOptions, Box<io::Write>), String> {
        let mut options: BindgenOptions = Default::default();

        for lib in self.flag_link.drain(..) {
            options.links.push((lib, LinkType::Default));
        }

        for lib in self.flag_static_link.drain(..) {
            options.links.push((lib, LinkType::Static));
        }

        for lib in self.flag_framework_link.drain(..) {
            options.links.push((lib, LinkType::Framework));
        }

        let out = if let Some(ref path_name) = self.flag_o {
            let path = path::Path::new(path_name);
            let file = try!(fs::File::create(path).map_err(|_| {
                format!("Opening {} failed", path_name)
            }));
            Box::new(io::BufWriter::new(file)) as Box<io::Write>
        } else {
            Box::new(io::BufWriter::new(io::stdout())) as Box<io::Write>
        };

        options.match_pat.extend(self.flag_match.drain(..));
        options.builtins = self.flag_builtins;
        options.ignore_functions = self.flag_ignore_functions;
        options.enable_cxx_namespaces = self.flag_enable_cxx_namespaces;
        options.rename_types = !self.flag_no_type_renaming;
        options.fail_on_unknown_type = !self.flag_allow_unknown_types;
        options.emit_ast = self.flag_emit_clang_ast;
        options.msvc_mangling = self.flag_use_msvc_mangling;
        options.override_enum_ty = self.flag_override_enum_type;
        options.raw_lines.extend(self.flag_raw_line.drain(..));
        options.dtor_attrs.extend(self.flag_dtor_attr.drain(..));
        options.class_constants = !self.flag_no_class_constants;
        options.unstable_rust = !self.flag_no_unstable_rust;
        options.namespaced_constants = !self.flag_no_namespaced_constants;
        options.gen_bitfield_methods = !self.flag_no_bitfield_methods;
        options.ignore_methods = self.flag_ignore_methods;
        options.opaque_types.extend(self.flag_opaque_type.drain(..));
        options.blacklist_type.extend(self.flag_blacklist_type.drain(..));
        options.clang_args.extend(self.arg_clang_args.drain(..));
        options.clang_args.push(self.arg_input_header);

        Ok((options, out))
    }
}

pub fn main() {
    let mut bind_args: Vec<_> = env::args().collect();

    if let Some(clang) = clang_sys::support::Clang::find(None) {
        let has_clang_args = bind_args.iter().rposition(|arg| *arg == "--").is_some();
        if !has_clang_args {
            bind_args.push("--".to_owned());
        }

        // TODO: distinguish C and C++ paths? C++'s should be enough, I guess.
        for path in clang.cpp_search_paths.into_iter() {
            if let Ok(path) = path.into_os_string().into_string() {
                bind_args.push("-isystem".to_owned());
                bind_args.push(path);
            }
        }
    }

    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.argv(bind_args.iter()).decode())
        .unwrap_or_else(|e| e.exit());

    let logger = StdLogger;
    let result: ParseResult<_> = args.into();
    let (options, out) = result.unwrap_or_else(|msg| {
        logger.error(&msg);
        exit(-1);
    });

    match Bindings::generate(&options, Some(&logger as &Logger), None) {
        Ok(bindings) => match bindings.write(out) {
            Ok(()) => (),
            Err(e) => {
                logger.error(&format!("Unable to write bindings to file. {}", e));
                exit(-1);
            }
        },
        Err(()) => {
            logger.error("Failed to generate bindings".into());
            exit(-1);
        }
    }
}
