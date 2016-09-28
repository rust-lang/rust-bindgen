#![crate_name = "bindgen"]
#![crate_type = "dylib"]

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

// To avoid rather annoying warnings when matching with CXCursor_xxx as a
// constant.
#![allow(non_upper_case_globals)]

extern crate syntex_syntax as syntax;
extern crate aster;
extern crate quasi;
extern crate clang_sys;
extern crate libc;
extern crate regex;
#[macro_use]
extern crate log;

mod clangll;
mod clang;
mod ir;
mod parse;
mod regex_set;
mod codegen {
    include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}

use std::borrow::Borrow;
use std::io::{self, Write};
use std::fs::OpenOptions;
use std::path::Path;
use std::marker;
use std::collections::HashSet;

use syntax::ast;
use syntax::codemap::{DUMMY_SP, Span};
use syntax::print::pprust;
use syntax::print::pp::eof;
use syntax::ptr::P;

use ir::context::BindgenContext;
use ir::item::{Item, ItemId};
use parse::{ClangItemParser, ParseError};
use regex_set::RegexSet;

#[derive(Debug)]
pub struct Builder<'a> {
    options: BindgenOptions,
    // TODO: Before the logger was here, do we still want the lifetime?
    phantom: marker::PhantomData<&'a ()>,
}

pub fn builder<'a>() -> Builder<'a> {
    Default::default()
}

impl<'a> Builder<'a> {
    pub fn header<T: Into<String>>(&mut self, header: T) -> &mut Self {
        self.clang_arg(header)
    }

    pub fn match_pat<T: Into<String>>(&mut self, arg: T) -> &mut Self {
        self.options.match_pat.insert(arg.into());
        self
    }

    pub fn hide_type<T: Into<String>>(&mut self, arg: T) -> &mut Self {
        self.options.hidden_types.insert(arg.into());
        self
    }

    pub fn opaque_type<T: Into<String>>(&mut self, arg: T) -> &mut Self {
        self.options.opaque_types.insert(arg.into());
        self
    }

    pub fn whitelisted_type<T: Borrow<str>>(&mut self, arg: &T) -> &mut Self {
        self.options.whitelisted_types.insert(arg);
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

    pub fn no_unstable_rust(&mut self) -> &mut Self {
        self.options.unstable_rust = false;
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

    pub fn disable_class_constants(&mut self) -> &mut Self {
        self.options.class_constants = false;
        self
    }

    pub fn generate(self) -> Result<Bindings, ()> {
        Bindings::generate(self.options, None)
    }
}

impl<'a> Default for Builder<'a> {
    fn default() -> Builder<'a> {
        Builder {
            options: Default::default(),
            phantom: marker::PhantomData,
        }
    }
}

/// Deprecated - use a `Builder` instead
#[derive(Debug)]
pub struct BindgenOptions {
    pub match_pat: HashSet<String>,
    pub hidden_types: HashSet<String>,
    pub opaque_types: HashSet<String>,
    pub whitelisted_types: RegexSet,
    pub whitelisted_functions: RegexSet,
    pub whitelisted_vars: RegexSet,
    pub builtins: bool,
    pub rust_enums: bool,
    pub links: Vec<(String, LinkType)>,
    pub emit_ast: bool,
    pub ignore_functions: bool,
    pub ignore_methods: bool,
    /// Whether to generate code for inline functions. If you do this, you need
    /// to make sure you compile the C++ code with a flag like
    /// -fkeep-inline-functions
    pub keep_inline_functions: bool,
    pub gen_bitfield_methods: bool,
    pub fail_on_unknown_type: bool,
    pub enable_cxx_namespaces: bool,
    pub rename_types: bool,
    pub derive_debug: bool,
    /// Generate or not only stable rust.
    pub unstable_rust: bool,
    /// Whether to generate C++ class constants.
    pub class_constants: bool,
    /// Wether to generate names that are **directly** under namespaces.
    pub namespaced_constants: bool,
    /// Whether to use msvc mangling rules
    pub msvc_mangling: bool,
    pub override_enum_ty: String,
    pub raw_lines: Vec<String>,
    /// Attributes for a type with destructor
    pub dtor_attrs: Vec<String>,
    pub clang_args: Vec<String>,
}

impl Default for BindgenOptions {
    fn default() -> BindgenOptions {
        BindgenOptions {
            match_pat: Default::default(),
            hidden_types: Default::default(),
            opaque_types: Default::default(),
            whitelisted_types: Default::default(),
            whitelisted_functions: Default::default(),
            whitelisted_vars: Default::default(),
            builtins: false,
            rust_enums: true,
            links: vec![],
            emit_ast: false,
            ignore_functions: false,
            ignore_methods: false,
            keep_inline_functions: false,
            gen_bitfield_methods: true,
            fail_on_unknown_type: true,
            rename_types: true,
            derive_debug: true,
            enable_cxx_namespaces: false,
            override_enum_ty: "".to_string(),
            unstable_rust: true,
            class_constants: true,
            namespaced_constants: true,
            msvc_mangling: false,
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

#[derive(Debug, Clone)]
pub struct Bindings {
    module: ast::Mod,
    raw_lines: Vec<String>,
}

impl Bindings {
    /// Deprecated - use a `Builder` instead
    pub fn generate(options: BindgenOptions, span: Option<Span>) -> Result<Bindings, ()> {
        let span = span.unwrap_or(DUMMY_SP);

        let mut context = BindgenContext::new(options);
        parse(&mut context);

        let module = ast::Mod {
            inner: span,
            items: codegen::codegen(&mut context),
        };

        Ok(Bindings {
            module: module,
            raw_lines: context.options().raw_lines.clone(),
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

/// Determines whether the given cursor is in any of the files matched by the
/// options.
fn filter_builtins(ctx: &BindgenContext, cursor: &clang::Cursor) -> bool {
    let (file, _, _, _) = cursor.location().location();

    match file.name() {
        None => ctx.options().builtins,
        Some(..) => true,
    }
}

pub fn parse_one(ctx: &mut BindgenContext,
                 cursor: clang::Cursor,
                 parent: Option<ItemId>,
                 children: &mut Vec<ItemId>) -> clangll::Enum_CXVisitorResult {
    if !filter_builtins(ctx, &cursor) {
        return CXChildVisit_Continue
    }

    use clangll::CXChildVisit_Continue;
    match Item::parse(cursor, parent, ctx) {
        Ok(id) => children.push(id),
        Err(ParseError::Continue) => {},
        Err(ParseError::Recurse) => {
            cursor.visit(|child, _| parse_one(ctx, *child, parent, children));
        }
    }
    CXChildVisit_Continue
}

fn parse(context: &mut BindgenContext) {
    use clang::Diagnostic;
    use clangll::*;

    for d in context.translation_unit().diags().iter() {
        let msg = d.format(Diagnostic::default_opts());
        let is_err = d.severity() >= CXDiagnostic_Error;
        println!("{}, err: {}", msg, is_err);
    }

    let cursor = context.translation_unit().cursor();
    if context.options().emit_ast {
        cursor.visit(|cur, _| clang::ast_dump(cur, 0));
    }

    let root = context.root_module();
    context.with_module(root, |context, children| {
        cursor.visit(|cursor, _| parse_one(context, *cursor, None, children))
    });

    assert!(context.current_module() == context.root_module(),
            "How did this happen?");
}
