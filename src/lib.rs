#![crate_name = "bindgen"]
#![crate_type = "dylib"]

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![warn(missing_docs)]

#![doc(html_root_url = "https://crabtw.github.io/rust-bindgen/")]


//! TODO: add some doc

extern crate clang_sys;
extern crate syntex_syntax as syntax;
extern crate libc;
#[macro_use]
extern crate log;
extern crate cexpr;

use std::collections::HashSet;
use std::default::Default;
use std::io::{self, Write};
use std::fs::OpenOptions;
use std::path::Path;

use syntax::ast;
use syntax::codemap::{DUMMY_SP, Span};
use syntax::print::pprust;
use syntax::print::pp::eof;
use syntax::ptr::P;

use types::Global;

use clang_sys::support::Clang;

mod types;
mod clang;
mod gen;
mod parser;

/// A builder to generate bindings.
#[derive(Debug, Clone)]
pub struct Builder<'a> {
    options: BindgenOptions,
    logger: Option<&'a Logger>,
}

impl<'a> Builder<'a> {
    /// Returns a new builder for the C header to parse.
    pub fn new<T: Into<String>>(header: T) -> Builder<'a> {
        let mut builder = Builder {
            logger: None,
            options: Default::default(),
        };
        builder.clang_arg(header);
        builder
    }

    /// Add a pattern to filter which file to generate a binding for.
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

    /// Control if we should use the c builtins like `__va_list`.
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

    /// Overrides the type used to represent a C enum.
    pub fn override_enum_ty<T: Into<String>>(&mut self, ty: T) -> &mut Self {
        self.options.override_enum_ty = ty.into();
        self
    }

    /// Set the prefix to remove from all the symbols, like `libfoo_`.
    pub fn remove_prefix<T: Into<String>>(&mut self, ty: T) -> &mut Self {
        self.options.remove_prefix = ty.into();
        self
    }

    /// Controls if bindgen should also print the parsed AST (for debug).
    pub fn emit_ast(&mut self, value: bool) -> &mut Self {
        self.options.emit_ast = value;
        self
    }

    /// Defines if we should use `std` or `core` for `Option` and such.
    pub fn use_core(&mut self, value: bool) -> &mut Self {
        self.options.use_core = value;
        self
    }

    /// Sets the prefix to use for c_void and others.
    pub fn ctypes_prefix<T: Into<Vec<String>>>(&mut self, prefix: T) -> &mut Self {
        self.options.ctypes_prefix = prefix.into();
        self
    }

    /// Defines if we should convert float and double to f32 and f64.
    ///
    /// The format is [not defined](https://en.wikipedia.org/wiki/C_data_types#Basic_types),
    /// but is the same as in rust in all the supported platforms.
    pub fn dont_convert_floats(&mut self) -> &mut Self {
        self.options.convert_floats = false;
        self
    }

    /// Turn macros definitions into const definitions, when possible
    pub fn convert_macros(&mut self, value: bool) -> &mut Self {
        self.options.convert_macros = value;
        self
    }

    /// When converting macros, convert integers that would fit in a `u8`,
    /// `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64` to the corresponding
    /// named C type the supplied list.
    ///
    /// The `Iterator` must return exactly 8 items.
    pub fn macro_int_types<T: Into<String>, I: Iterator<Item=T>>(&mut self, mut types: I) -> &mut Self {
        self.options.macro_int_types = (
            types.next().expect("Not enough types in macro_int_types").into(),
            types.next().expect("Not enough types in macro_int_types").into(),
            types.next().expect("Not enough types in macro_int_types").into(),
            types.next().expect("Not enough types in macro_int_types").into(),
            types.next().expect("Not enough types in macro_int_types").into(),
            types.next().expect("Not enough types in macro_int_types").into(),
            types.next().expect("Not enough types in macro_int_types").into(),
            types.next().expect("Not enough types in macro_int_types").into(),
        );
        if let Some(_)=types.next() {
            panic!("Additional type specified in macro_int_types")
        }
        self
    }

    /// Generate the binding using the options previously set.
    pub fn generate(&self) -> Result<Bindings, ()> {
        Bindings::generate(&self.options, self.logger, None)
    }
}

#[derive(Debug, Clone)]
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
    /// The prefix to use for the c types like c_void.
    ///
    /// Default: ["std", "os", "raw"]
    pub ctypes_prefix: Vec<String>,
    /// Defines if we should use `std` or `core` for `Option` and such.
    pub use_core: bool,
    /// Prefix to remove from all the symbols, like `libfoo_`.
    pub remove_prefix: String,
    /// See `Builder::convert_floats`.
    pub convert_floats: bool,
    pub convert_macros: bool,
    // would use Array here but that requires Copy
    pub macro_int_types: (String,String,String,String,String,String,String,String),
}

impl Default for BindgenOptions {
    fn default() -> BindgenOptions {
        let clang = Clang::find(None).expect("No clang found, is it installed?");
        let mut args = Vec::new();
        for dir in clang.c_search_paths {
            args.push("-idirafter".to_owned());
            args.push(dir.to_str().unwrap().to_owned());
        }
        if cfg!(target_os="macos") {
            args.push("-U__BLOCKS__".to_owned());
        }
        BindgenOptions {
            match_pat: Vec::new(),
            builtins: false,
            rust_enums: true,
            links: Vec::new(),
            emit_ast: false,
            fail_on_unknown_type: true,
            override_enum_ty: "".to_owned(),
            clang_args: args,
            derive_debug: true,
            ctypes_prefix: vec!["std".into(), "os".into(), "raw".into()],
            use_core: false,
            remove_prefix: String::new(),
            convert_floats: true,
            convert_macros: false,
            macro_int_types: ("uchar".to_owned(),"ushort".to_owned(),"uint".to_owned(),"ulonglong".to_owned(),"schar".to_owned(),"sshort".to_owned(),"sint".to_owned(),"slonglong".to_owned())
        }
    }
}

/// Type of the link to the library which binding is generating.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LinkType {
    /// Do a static link to the library.
    Static,
    /// Do a dynamic link to the library.
    Dynamic,
    /// Link to a MacOS Framework.
    Framework,
}

/// Trait used internaly to log things with context like the C file line number.
pub trait Logger: std::fmt::Debug {
    /// Defaults to `error!()`.
    fn error(&self, msg: &str) {
        error!("{}", msg);
    }

    /// Defaults to `warn!()`.
    fn warn(&self, msg: &str) {
        warn!("{}", msg);
    }
}

/// Contains the generated code.
#[derive(Clone)]
pub struct Bindings {
    module: ast::Mod,
    attributes: Vec<ast::Attribute>,
}

impl Bindings {
    /// Deprecated - use a `Builder` instead
    #[doc(hidden)]
    pub fn generate(options: &BindgenOptions,
                    logger: Option<&Logger>,
                    span: Option<Span>)
                    -> Result<Bindings, ()> {
        let l = DummyLogger;
        let logger = match logger {
            Some(l) => l,
            None => &l as &Logger,
        };

        let span = match span {
            Some(s) => s,
            None => DUMMY_SP,
        };

        let globals = try!(parse_headers(options, logger));

        let (m, attrs) = gen::gen_mod(options, globals, span);
        let module = ast::Mod {
            inner: span,
            items: m,
        };

        Ok(Bindings {
            module: module,
            attributes: attrs,
        })
    }

    /// Get the generated code AST.
    pub fn into_ast(self) -> Vec<P<ast::Item>> {
        self.module.items
    }

    /// Get the generated code in a String.
    pub fn to_string(&self) -> String {
        let mut mod_str = Vec::new();
        {
            let ref_writer = Box::new(mod_str.by_ref()) as Box<Write>;
            self.write(ref_writer).expect("Could not write bindings to string");
        }
        String::from_utf8(mod_str).unwrap()
    }

    /// Write the generated code in a file.
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let file = try!(OpenOptions::new().write(true).truncate(true).create(true).open(path));
        self.write(Box::new(file))
    }

    // https://github.com/Manishearth/rust-clippy/issues/740
    #[cfg_attr(feature = "clippy", allow(needless_lifetimes))]
    /// Write the generated code in the provided `Write` object.
    pub fn write<'a>(&self, mut writer: Box<Write + 'a>) -> io::Result<()> {
        try!(writer.write("/* automatically generated by rust-bindgen */\n\n".as_bytes()));
        let mut ps = pprust::rust_printer(writer);
        try!(ps.print_mod(&self.module, &self.attributes));
        try!(ps.print_remaining_comments());
        try!(eof(&mut ps.s));
        ps.s.out.flush()
    }
}


#[derive(Debug)]
struct DummyLogger;

impl Logger for DummyLogger {}

fn parse_headers(options: &BindgenOptions, logger: &Logger) -> Result<Vec<Global>, ()> {
    fn str_to_ikind(s: &str) -> Option<types::IKind> {
        match s {
            "uchar" => Some(types::IUChar),
            "schar" => Some(types::ISChar),
            "ushort" => Some(types::IUShort),
            "sshort" => Some(types::IShort),
            "uint" => Some(types::IUInt),
            "sint" => Some(types::IInt),
            "ulong" => Some(types::IULong),
            "slong" => Some(types::ILong),
            "ulonglong" => Some(types::IULongLong),
            "slonglong" => Some(types::ILongLong),
            _ => None,
        }
    }

    let m_ty=parser::MacroTypes{
        t_u8:  str_to_ikind(&options.macro_int_types.0).expect("Invalid C type specified for u8"),
        t_u16: str_to_ikind(&options.macro_int_types.1).expect("Invalid C type specified for u16"),
        t_u32: str_to_ikind(&options.macro_int_types.2).expect("Invalid C type specified for u32"),
        t_u64: str_to_ikind(&options.macro_int_types.3).expect("Invalid C type specified for u64"),
        t_i8:  str_to_ikind(&options.macro_int_types.4).expect("Invalid C type specified for i8"),
        t_i16: str_to_ikind(&options.macro_int_types.5).expect("Invalid C type specified for i16"),
        t_i32: str_to_ikind(&options.macro_int_types.6).expect("Invalid C type specified for i32"),
        t_i64: str_to_ikind(&options.macro_int_types.7).expect("Invalid C type specified for i64"),
    };

    let clang_opts = parser::ClangParserOptions {
        builtin_names: builtin_names(),
        builtins: options.builtins,
        match_pat: options.match_pat.clone(),
        emit_ast: options.emit_ast,
        fail_on_unknown_type: options.fail_on_unknown_type,
        override_enum_ty: str_to_ikind(&options.override_enum_ty[..]),
        clang_args: options.clang_args.clone(),
        macros: options.convert_macros,
        macro_types: m_ty,
    };

    parser::parse(clang_opts, logger)
}

fn builtin_names() -> HashSet<String> {
    let mut names = HashSet::new();
    let keys = ["__va_list_tag", "__va_list", "__builtin_va_list"];

    for s in &keys {
        names.insert((*s).to_owned());
    }

    names
}

#[test]
fn builder_state() {
    let logger = DummyLogger;
    let mut build = Builder::new("example.h");
    build.link("m", LinkType::Static)
         .log(&logger);
    assert!(build.logger.is_some());
    assert!(build.options.clang_args.binary_search(&"example.h".to_owned()).is_ok());
    assert!(build.options.links.binary_search(&("m".to_owned(), LinkType::Static)).is_ok());
}
