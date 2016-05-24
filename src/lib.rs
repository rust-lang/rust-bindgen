#![crate_name = "bindgen"]
#![crate_type = "dylib"]

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate clang_sys;
extern crate syntex_syntax as syntax;
extern crate libc;
#[macro_use] extern crate log;

use std::collections::HashSet;
use std::default::Default;
use std::io::{Write, self};
use std::fs::OpenOptions;
use std::path::{Path, self};
use std::{env, fs};

use syntax::ast;
use syntax::codemap::{DUMMY_SP, Span};
use syntax::print::pprust;
use syntax::print::pp::eof;
use syntax::ptr::P;

use types::Global;

mod types;
mod clang;
mod gen;
mod parser;

#[derive(Clone)]
pub struct Builder<'a> {
    options: BindgenOptions,
    logger: Option<&'a Logger>
}

pub fn builder<'a>() -> Builder<'a> {
    Default::default()
}

impl<'a> Builder<'a> {
    /// Add a C header to parse.
    pub fn header<T: Into<String>>(&mut self, header: T) -> &mut Self {
        self.clang_arg(header)
    }

    pub fn match_pat<T: Into<String>>(&mut self, arg: T) -> &mut Self {
        self.options.match_pat.push(arg.into());
        self
    }

    /// Add a clang CLI argument.
    pub fn clang_arg<T: Into<String>>(&mut self, arg: T) -> &mut Self {
        self.options.clang_args.push(arg.into());
        self
    }

    /// Add a library to link.
    pub fn link<T: Into<String>>(&mut self, library: T, link_type: LinkType) -> &mut Self {
        self.options.links.push((library.into(), link_type));
        self
    }

    /// Force bindgen to exit if a type is not recognized.
    pub fn forbid_unknown_types(&mut self) -> &mut Self {
        self.options.fail_on_unknown_type = true;
        self
    }

    pub fn builtins(&mut self) -> &mut Self {
        self.options.builtins = true;
        self
    }

    /// Control if the generated structs will derive Debug.
    pub fn derive_debug(&mut self, derive_debug: bool) -> &mut Self {
        self.options.derive_debug = derive_debug;
        self
    }

    /// Control if bindgen should convert the C enums to rust enums or rust constants.
    pub fn rust_enums(&mut self, value: bool) -> &mut Self {
        self.options.rust_enums = value;
        self
    }

    /// Set the logger to use.
    pub fn log(&mut self, logger: &'a Logger) -> &mut Self {
        self.logger = Some(logger);
        self
    }

    pub fn override_enum_ty<T: Into<String>>(&mut self, ty: T) -> &mut Self {
        self.options.override_enum_ty = ty.into();
        self
    }

    /// Controls if bindgen should also print the parsed AST (for debug).
    pub fn emit_ast(&mut self, value: bool) -> &mut Self {
        self.options.emit_ast = value;
        self
    }

    /// Generate the binding using the options previously set.
    pub fn generate(&self) -> Result<Bindings, ()> {
        Bindings::generate(&self.options, self.logger, None)
    }
}

impl<'a> Default for Builder<'a> {
    fn default() -> Builder<'a> {
        Builder {
            logger: None,
            options: Default::default()
        }
    }
}

#[derive(Clone)]
/// Deprecated - use a `Builder` instead
#[doc(hidden)]
pub struct BindgenOptions {
    pub match_pat: Vec<String>,
    pub builtins: bool,
    pub rust_enums: bool,
    pub links: Vec<(String, LinkType)>,
    pub emit_ast: bool,
    pub fail_on_unknown_type: bool,
    pub override_enum_ty: String,
    pub clang_args: Vec<String>,
    pub derive_debug: bool,
}

impl Default for BindgenOptions {
    fn default() -> BindgenOptions {
        BindgenOptions {
            match_pat: Vec::new(),
            builtins: false,
            rust_enums: true,
            links: Vec::new(),
            emit_ast: false,
            fail_on_unknown_type: true,
            override_enum_ty: "".to_owned(),
            clang_args: match get_include_dir() {
                Some(path) => vec!("-idirafter".to_owned(), path),
                None => Vec::new()
            },
            derive_debug: true
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LinkType {
    Static,
    Dynamic,
    Framework
}

pub trait Logger {
    fn error(&self, msg: &str);
    fn warn(&self, msg: &str);
}

#[derive(Clone)]
pub struct Bindings {
    module: ast::Mod
}

impl Bindings {
    /// Deprecated - use a `Builder` instead
    #[doc(hidden)]
    pub fn generate(options: &BindgenOptions, logger: Option<&Logger>, span: Option<Span>) -> Result<Bindings, ()> {
        let l = DummyLogger;
        let logger = match logger {
            Some(l) => l,
            None => &l as &Logger
        };

        let span = match span {
            Some(s) => s,
            None => DUMMY_SP
        };

        let globals = try!(parse_headers(options, logger));

        let module = ast::Mod {
            inner: span,
            items: gen::gen_mod(options, globals, span)
        };

        Ok(Bindings {
            module: module
        })
    }

    pub fn into_ast(self) -> Vec<P<ast::Item>> {
        self.module.items
    }

    pub fn to_string(&self) -> String {
        let mut mod_str = Vec::new();
        {
            let ref_writer = Box::new(mod_str.by_ref()) as Box<Write>;
            self.write(ref_writer).expect("Could not write bindings to string");
        }
        String::from_utf8(mod_str).unwrap()
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let file = try!(OpenOptions::new().write(true).truncate(true).create(true).open(path));
        self.write(Box::new(file))
    }

    // https://github.com/Manishearth/rust-clippy/issues/740
    #[cfg_attr(feature = "clippy", allow(needless_lifetimes))]
    pub fn write<'a>(&self, mut writer: Box<Write + 'a>) -> io::Result<()> {
        try!(writer.write("/* automatically generated by rust-bindgen */\n\n".as_bytes()));
        let mut ps = pprust::rust_printer(writer);
        try!(ps.print_mod(&self.module, &[]));
        try!(ps.print_remaining_comments());
        try!(eof(&mut ps.s));
        ps.s.out.flush()
    }
}


struct DummyLogger;

impl Logger for DummyLogger {
    fn error(&self, _msg: &str) { }
    fn warn(&self, _msg: &str) { }
}

fn parse_headers(options: &BindgenOptions, logger: &Logger) -> Result<Vec<Global>, ()> {
    fn str_to_ikind(s: &str) -> Option<types::IKind> {
        match s {
            "uchar"     => Some(types::IUChar),
            "schar"     => Some(types::ISChar),
            "ushort"    => Some(types::IUShort),
            "sshort"    => Some(types::IShort),
            "uint"      => Some(types::IUInt),
            "sint"      => Some(types::IInt),
            "ulong"     => Some(types::IULong),
            "slong"     => Some(types::ILong),
            "ulonglong" => Some(types::IULongLong),
            "slonglong" => Some(types::ILongLong),
            _           => None,
        }
    }

    let clang_opts = parser::ClangParserOptions {
        builtin_names: builtin_names(),
        builtins: options.builtins,
        match_pat: options.match_pat.clone(),
        emit_ast: options.emit_ast,
        fail_on_unknown_type: options.fail_on_unknown_type,
        override_enum_ty: str_to_ikind(&options.override_enum_ty[..]),
        clang_args: options.clang_args.clone(),
    };

    parser::parse(clang_opts, logger)
}

fn builtin_names() -> HashSet<String> {
    let mut names = HashSet::new();
    let keys = [
        "__va_list_tag",
        "__va_list",
        "__builtin_va_list",
    ];

    for s in &keys {
        names.insert((*s).to_owned());
    }

    names
}

#[test]
fn builder_state()
{
    let logger = DummyLogger;
    let mut build = builder();
    {
        build.header("example.h");
        build.link("m", LinkType::Static);
        build.log(&logger);
    }
    assert!(build.logger.is_some());
    assert!(build.options.clang_args.binary_search(&"example.h".to_owned()).is_ok());
    assert!(build.options.links.binary_search(&("m".to_owned(), LinkType::Static)).is_ok());
}

// Get the first directory in PATH that contains a file named "clang".
fn get_clang_dir() -> Option<path::PathBuf>{
    if let Some(paths) = env::var_os("PATH") {
        for mut path in env::split_paths(&paths) {
            path.push("clang");
            if let Ok(real_path) = fs::canonicalize(&path) {
                if fs::metadata(&real_path).iter().any(|m| m.is_file()) &&
                    real_path
                        .file_name()
                        .and_then(|f| f.to_str())
                        .iter()
                        .any(|&f| f.starts_with("clang")) {
                    if let Some(dir) = real_path.parent() {
                        return Some(dir.to_path_buf())
                    }
                }
            }
        }
    }
    None
}

// Try to find the directory that contains clang's bundled headers. Clang itself does something
// very similar: it takes the parent directory of the current executable, appends
// "../lib/clang/<VERSIONSTRING>/include". We have two problems emulating this behaviour:
// * We don't have a very good way of finding the clang executable, but can fake this by
//   searching $PATH and take one directory that contains "clang".
// * We don't have access to <VERSIONSTRING>. There is clang_getClangVersion(), but it returns
//   a human-readable description string which is not guaranteed to be stable and a pain to parse.
//   We work around that by just taking the first directory in ../lib/clang and hope it's the
//   current version.
// TODO: test if this works on Windows at all.
#[doc(hidden)]
pub fn get_include_dir() -> Option<String> {
    match get_clang_dir() {
        Some(mut p) => {
            p.push("..");
            p.push("lib");
            p.push("clang");

            let dir_iter = match fs::read_dir(p) {
                Ok(dir_iter) => dir_iter,
                _ => return None
            };
            for dir in dir_iter {
                match dir {
                    Ok(dir) => {
                        // Let's take the first dir. In my case, there's only one directory
                        // there anyway.
                        let mut p = dir.path();
                        p.push("include");
                        match p.into_os_string().into_string() {
                            Ok(s) => return Some(s),
                            // We found the directory, but can't access it as it contains
                            // invalid unicode.
                            _ => return None,
                        }
                    }
                    _ => return None,
                }
            }
            None
        }
        None => None,
    }
}
