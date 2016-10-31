//! Generate Rust bindings for C and C++ libraries.
//!
//! Provide a C/C++ header file, receive Rust FFI code to call into C/C++
//! functions and use types defined in the header.
//!
//! See the [Builder](./struct.Builder.html) struct for usage.

#![crate_name = "bindgen"]
#![crate_type = "dylib"]

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#![deny(missing_docs)]
#![deny(warnings)]

// We internally use the deprecated BindgenOptions all over the place. Once we
// remove its `pub` declaration, we can un-deprecate it and remove this pragma.
#![allow(deprecated)]

// To avoid rather annoying warnings when matching with CXCursor_xxx as a
// constant.
#![allow(non_upper_case_globals)]

#[macro_use]
extern crate cfg_if;
extern crate syntex_syntax as syntax;
extern crate aster;
extern crate quasi;
extern crate clang_sys;
extern crate libc;
extern crate regex;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

// A macro to declare an internal module for which we *must* provide
// documentation for. If we are building with the "_docs" feature, then the
// module is declared public, and our `#![deny(missing_docs)]` pragma applies to
// it. This feature is used in CI, so we won't let anything slip by
// undocumented. Normal builds, however, will leave the module private, so that
// we don't expose internals to library consumers.
macro_rules! doc_mod {
    ($m:ident) => {
        cfg_if! {
            if #[cfg(feature = "_docs")] {
                pub mod $m;
            } else {
                mod $m;
            }
        }
    };
}

mod clangll;
doc_mod!(clang);
doc_mod!(ir);
doc_mod!(parse);
doc_mod!(regex_set);

mod codegen {
    include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}

use std::borrow::Borrow;
use std::io::{self, Write};
use std::fs::OpenOptions;
use std::path::Path;
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

/// Configure and generate Rust bindings for a C/C++ header.
///
/// This is the main entry point to the library.
///
/// ```ignore
/// use bindgen::builder;
///
/// // Configure and generate bindings.
/// let bindings = try!(builder().header("path/to/input/header")
///                              .whitelisted_type("SomeCoolClass")
///                              .whitelisted_function("do_some_cool_thing")
///                              .generate());
///
/// // Write the generated bindings to an output file.
/// try!(bindings.write_to_file("path/to/output.rs"));
/// ```
#[derive(Debug,Default)]
pub struct Builder {
    options: BindgenOptions,
}

/// Construct a new [`Builder`](./struct.Builder.html).
pub fn builder() -> Builder {
    Default::default()
}

impl Builder {
    /// Set the input C/C++ header.
    pub fn header<T: Into<String>>(self, header: T) -> Builder {
        self.clang_arg(header)
    }

    /// Hide the given type from the generated bindings.
    pub fn hide_type<T: Into<String>>(mut self, arg: T) -> Builder {
        self.options.hidden_types.insert(arg.into());
        self
    }

    /// Treat the given type as opaque in the generated bindings.
    pub fn opaque_type<T: Into<String>>(mut self, arg: T) -> Builder {
        self.options.opaque_types.insert(arg.into());
        self
    }

    /// Whitelist the given type so that it (and all types that it transitively
    /// refers to) appears in the generated bindings.
    pub fn whitelisted_type<T: Borrow<str>>(mut self, arg: T) -> Builder {
        self.options.whitelisted_types.insert(&arg);
        self
    }

    /// Whitelist the given function so that it (and all types that it
    /// transitively refers to) appears in the generated bindings.
    pub fn whitelisted_function<T: Borrow<str>>(mut self, arg: T) -> Builder {
        self.options.whitelisted_functions.insert(&arg);
        self
    }

    /// Whitelist the given variable so that it (and all types that it
    /// transitively refers to) appears in the generated bindings.
    pub fn whitelisted_var<T: Borrow<str>>(mut self, arg: T) -> Builder {
        self.options.whitelisted_vars.insert(&arg);
        self
    }

    /// Add a string to prepend to the generated bindings. The string is passed
    /// through without any modification.
    pub fn raw_line<T: Into<String>>(mut self, arg: T) -> Builder {
        self.options.raw_lines.push(arg.into());
        self
    }

    /// Add an argument to be passed straight through to clang.
    pub fn clang_arg<T: Into<String>>(mut self, arg: T) -> Builder {
        self.options.clang_args.push(arg.into());
        self
    }

    /// Make the generated bindings link the given shared library.
    pub fn link<T: Into<String>>(mut self, library: T) -> Builder {
        self.options.links.push((library.into(), LinkType::Default));
        self
    }

    /// Make the generated bindings link the given static library.
    pub fn link_static<T: Into<String>>(mut self, library: T) -> Builder {
        self.options.links.push((library.into(), LinkType::Static));
        self
    }

    /// Make the generated bindings link the given framework.
    pub fn link_framework<T: Into<String>>(mut self, library: T) -> Builder {
        self.options.links.push((library.into(), LinkType::Framework));
        self
    }

    /// Emit bindings for builtin definitions (for example `__builtin_va_list`)
    /// in the generated Rust.
    pub fn emit_builtins(mut self) -> Builder {
        self.options.builtins = true;
        self
    }

    /// Avoid generating any unstable Rust in the generated bindings.
    pub fn no_unstable_rust(mut self) -> Builder {
        self.options.unstable_rust = false;
        self
    }

    /// Generate the Rust bindings using the options built up thus far.
    pub fn generate(self) -> Result<Bindings, ()> {
        Bindings::generate(self.options, None)
    }
}

/// Configuration options for generated bindings.
///
/// Deprecated: use a `Builder` instead.
#[derive(Debug)]
#[deprecated]
pub struct BindgenOptions {
    /// The set of types that have been blacklisted and should not appear
    /// anywhere in the generated code.
    pub hidden_types: HashSet<String>,

    /// The set of types that should be treated as opaque structures in the
    /// generated code.
    pub opaque_types: HashSet<String>,

    /// The set of types that we should have bindings for in the generated
    /// code.
    ///
    /// This includes all types transitively reachable from any type in this
    /// set. One might think of whitelisted types/vars/functions as GC roots,
    /// and the generated Rust code as including everything that gets marked.
    pub whitelisted_types: RegexSet,

    /// Whitelisted functions. See docs for `whitelisted_types` for more.
    pub whitelisted_functions: RegexSet,

    /// Whitelisted variables. See docs for `whitelisted_types` for more.
    pub whitelisted_vars: RegexSet,

    /// Whether we should generate builtins or not.
    pub builtins: bool,

    /// The set of libraries we should link in the generated Rust code.
    pub links: Vec<(String, LinkType)>,

    /// True if we should dump the Clang AST for debugging purposes.
    pub emit_ast: bool,

    /// True if we should ignore functions and only generate bindings for
    /// structures, types, and methods.
    pub ignore_functions: bool,

    /// True if we should avoid generating bindings for methods, and instead
    /// just generate code for structures and types.
    pub ignore_methods: bool,

    /// True if we should emulate C++ namespaces with Rust modules in the
    /// generated bindings.
    pub enable_cxx_namespaces: bool,

    /// True if we shold derive Debug trait implementations for C/C++ structures
    /// and types.
    pub derive_debug: bool,

    /// True if we can use unstable Rust code in the bindings, false if we
    /// cannot.
    pub unstable_rust: bool,

    /// True if we should generate constant names that are **directly** under
    /// namespaces.
    pub namespaced_constants: bool,

    /// True if we should use MSVC name mangling rules.
    pub msvc_mangling: bool,

    /// The set of raw lines to prepend to the generated Rust code.
    pub raw_lines: Vec<String>,

    /// The set of arguments to pass straight through to Clang.
    pub clang_args: Vec<String>,
}

impl Default for BindgenOptions {
    fn default() -> BindgenOptions {
        BindgenOptions {
            hidden_types: Default::default(),
            opaque_types: Default::default(),
            whitelisted_types: Default::default(),
            whitelisted_functions: Default::default(),
            whitelisted_vars: Default::default(),
            builtins: false,
            links: vec![],
            emit_ast: false,
            ignore_functions: false,
            ignore_methods: false,
            derive_debug: true,
            enable_cxx_namespaces: false,
            unstable_rust: true,
            namespaced_constants: true,
            msvc_mangling: false,
            raw_lines: vec![],
            clang_args: vec![],
        }
    }
}

/// The linking type to use with a given library.
///
/// TODO: #104: This is ignored at the moment, but shouldn't be.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LinkType {
    /// Use shared library linking. This is the default.
    Default,
    /// Use static linking.
    Static,
    /// The library is an OSX framework.
    Framework
}

/// Generated Rust bindings.
#[derive(Debug, Clone)]
pub struct Bindings {
    module: ast::Mod,
    raw_lines: Vec<String>,
}

impl Bindings {
    /// Generate bindings for the given options.
    ///
    /// Deprecated - use a `Builder` instead
    #[deprecated]
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

    /// Convert these bindings into a Rust AST.
    pub fn into_ast(self) -> Vec<P<ast::Item>> {
        self.module.items
    }

    /// Convert these bindings into source text (with raw lines prepended).
    pub fn to_string(&self) -> String {
        let mut mod_str = vec![];
        {
            let ref_writer = Box::new(mod_str.by_ref()) as Box<Write>;
            self.write(ref_writer).expect("Could not write bindings to string");
        }
        String::from_utf8(mod_str).unwrap()
    }

    /// Write these bindings as source text to a file.
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let file = try!(OpenOptions::new().write(true).truncate(true).create(true).open(path));
        self.write(Box::new(file))
    }

    /// Write these bindings as source text to the given `Write`able.
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

/// Determines whether the given cursor is in any of the files matched by the
/// options.
fn filter_builtins(ctx: &BindgenContext, cursor: &clang::Cursor) -> bool {
    let (file, _, _, _) = cursor.location().location();

    match file.name() {
        None => ctx.options().builtins,
        Some(..) => true,
    }
}

/// Parse one `Item` from the Clang cursor.
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

/// Parse the Clang AST into our `Item` internal representation.
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
