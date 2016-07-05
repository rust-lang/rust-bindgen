#![crate_name = "bindgen"]
#![crate_type = "dylib"]
#![feature(quote)]
#![feature(borrow_state)]

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate syntex_syntax as syntax;
extern crate clang_sys;
extern crate libc;
#[macro_use]
extern crate log;

use std::collections::HashSet;
use std::default::Default;
use std::io::{Write, self};
use std::fs::OpenOptions;
use std::path::Path;

use syntax::ast;
use syntax::codemap::{DUMMY_SP, Span};
use syntax::print::pprust;
use syntax::print::pp::eof;
use syntax::ptr::P;

use types::ModuleMap;

mod types;
mod clangll;
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
    pub fn header<T: Into<String>>(&mut self, header: T) -> &mut Self {
        self.clang_arg(header)
    }

    pub fn match_pat<T: Into<String>>(&mut self, arg: T) -> &mut Self {
        self.options.match_pat.push(arg.into());
        self
    }

    pub fn blacklist_type<T: Into<String>>(&mut self, arg: T) -> &mut Self {
        self.options.blacklist_type.push(arg.into());
        self
    }

    pub fn opaque_type<T: Into<String>>(&mut self, arg: T) -> &mut Self {
        self.options.opaque_types.push(arg.into());
        self
    }

    pub fn raw_line<T: Into<String>>(&mut self, arg: T) -> &mut Self {
        self.options.raw_lines.push(arg.into());
        self
    }

    pub fn clang_arg<T: Into<String>>(&mut self, arg: T) -> &mut Self {
        self.options.clang_args.push(arg.into());
        self
    }

    pub fn link<T: Into<String>>(&mut self, library: T) -> &mut Self {
        self.options.links.push((library.into(), LinkType::Default));
        self
    }

    pub fn link_static<T: Into<String>>(&mut self, library: T) -> &mut Self {
        self.options.links.push((library.into(), LinkType::Static));
        self
    }

    pub fn link_framework<T: Into<String>>(&mut self, library: T) -> &mut Self {
        self.options.links.push((library.into(), LinkType::Framework));
        self
    }

    pub fn dtor_attr<T: Into<String>>(&mut self, attr: T) -> &mut Self {
        self.options.dtor_attrs.push(attr.into());
        self
    }

    pub fn forbid_unknown_types(&mut self) -> &mut Self {
        self.options.fail_on_unknown_type = true;
        self
    }

    pub fn emit_builtins(&mut self) -> &mut Self {
        self.options.builtins = true;
        self
    }

    pub fn no_bitfield_methods(&mut self) -> &mut Self {
        self.options.gen_bitfield_methods = false;
        self
    }

    pub fn rust_enums(&mut self, value: bool) -> &mut Self {
        self.options.rust_enums = value;
        self
    }

    pub fn rename_types(&mut self, value: bool) -> &mut Self {
        self.options.rename_types = value;
        self
    }

    pub fn log(&mut self, logger: &'a Logger) -> &mut Self {
        self.logger = Some(logger);
        self
    }

    pub fn disable_class_constants(&mut self) -> &mut Self {
        self.options.class_constants = false;
        self
    }

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
pub struct BindgenOptions {
    pub match_pat: Vec<String>,
    pub blacklist_type: Vec<String>,
    pub opaque_types: Vec<String>,
    pub builtins: bool,
    pub rust_enums: bool,
    pub links: Vec<(String, LinkType)>,
    pub emit_ast: bool,
    pub ignore_functions: bool,
    pub gen_bitfield_methods: bool,
    pub fail_on_unknown_type: bool,
    pub enable_cxx_namespaces: bool,
    pub rename_types: bool,
    pub derive_debug: bool,
    /// Wether to generate C++ class constants.
    pub class_constants: bool,
    /// Wether to generate names that are **directly** under namespaces.
    pub namespaced_constants: bool,
    pub override_enum_ty: String,
    pub raw_lines: Vec<String>,
    /// Attributes for a type with destructor
    pub dtor_attrs: Vec<String>,
    pub clang_args: Vec<String>,
}

impl Default for BindgenOptions {
    fn default() -> BindgenOptions {
        BindgenOptions {
            match_pat: vec![],
            blacklist_type: vec![],
            opaque_types: vec![],
            builtins: false,
            rust_enums: true,
            links: vec![],
            emit_ast: false,
            ignore_functions: false,
            gen_bitfield_methods: true,
            fail_on_unknown_type: true,
            rename_types: true,
            derive_debug: true,
            enable_cxx_namespaces: false,
            override_enum_ty: "".to_string(),
            class_constants: true,
            namespaced_constants: true,
            raw_lines: vec![],
            dtor_attrs: vec![],
            clang_args: vec![],
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LinkType {
    Default,
    Static,
    Framework
}

pub trait Logger {
    fn error(&self, msg: &str);
    fn warn(&self, msg: &str);
}

#[derive(Debug, Clone)]
pub struct Bindings {
    module: ast::Mod,
    raw_lines: Vec<String>,
}

impl Bindings {
    /// Deprecated - use a `Builder` instead
    pub fn generate(options: &BindgenOptions, logger: Option<&Logger>, span: Option<Span>) -> Result<Bindings, ()> {
        let l = DummyLogger;
        let logger = logger.unwrap_or(&l as &Logger);

        let span = span.unwrap_or(DUMMY_SP);

        let module_map = try!(parse_headers(options, logger));

        let module = ast::Mod {
            inner: span,
            items: gen::gen_mods(&options.links[..],
                                 module_map,
                                 options.clone(),
                                 span)
        };

        Ok(Bindings {
            module: module,
            raw_lines: options.raw_lines.clone(),
        })
    }

    pub fn into_ast(self) -> Vec<P<ast::Item>> {
        self.module.items
    }

    pub fn to_string(&self) -> String {
        let mut mod_str = vec![];
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

        for line in self.raw_lines.iter() {
            try!(writer.write(line.as_bytes()));
            try!(writer.write("\n".as_bytes()));
        }
        if !self.raw_lines.is_empty() {
            try!(writer.write("\n".as_bytes()));
        }

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

fn parse_headers(options: &BindgenOptions, logger: &Logger) -> Result<ModuleMap, ()> {
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

    // TODO: Unify most of these with BindgenOptions?
    let clang_opts = parser::ClangParserOptions {
        builtin_names: builtin_names(),
        builtins: options.builtins,
        match_pat: options.match_pat.clone(),
        emit_ast: options.emit_ast,
        class_constants: options.class_constants,
        namespaced_constants: options.namespaced_constants,
        ignore_functions: options.ignore_functions,
        fail_on_unknown_type: options.fail_on_unknown_type,
        enable_cxx_namespaces: options.enable_cxx_namespaces,
        override_enum_ty: str_to_ikind(&options.override_enum_ty),
        clang_args: options.clang_args.clone(),
        opaque_types: options.opaque_types.clone(),
        blacklist_type: options.blacklist_type.clone(),
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
fn builder_state() {
    let logger = DummyLogger;
    let mut build = builder();
    {
        build.header("example.h");
        build.link_static("m");
        build.log(&logger);
    }
    assert!(build.logger.is_some());
    assert!(build.options.clang_args.binary_search(&"example.h".to_owned()).is_ok());
    assert!(build.options.links.binary_search(&("m".to_owned(), LinkType::Static)).is_ok());
}
