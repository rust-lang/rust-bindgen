#![crate_name = "bindgen"]
#![crate_type = "dylib"]
#![feature(globs, managed_boxes, quote, phase, plugin_registrar)]

extern crate syntax;
extern crate rustc;
extern crate libc;
#[phase(plugin, link)] extern crate log;

use std::collections::HashSet;
use std::default::Default;
use std::gc::Gc;

use syntax::ast;
use syntax::codemap::Span;
use rustc::plugin::Registry;

use types::Global;

mod types;
mod clangll;
mod clang;
mod gen;
mod parser;
mod macro;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("bindgen", macro::bindgen_macro);
}

pub struct BindgenOptions {
    pub match_pat: Vec<String>,
    pub abi: String,
    pub builtins: bool,
    pub links: Vec<(String, Option<String>)>,
    pub emit_ast: bool,
    pub fail_on_bitfield: bool,
    pub fail_on_unknown_type: bool,
    pub clang_args: Vec<String>,
}

impl Default for BindgenOptions {
    fn default() -> BindgenOptions {
        BindgenOptions {
            match_pat: Vec::new(),
            abi: "C".to_string(),
            builtins: false,
            links: Vec::new(),
            emit_ast: false,
            fail_on_bitfield: false,
            fail_on_unknown_type: false,
            clang_args: Vec::new()
        }
    }
}

pub trait Logger {
    fn error(&self, msg: &str);
    fn warn(&self, msg: &str);
}

pub fn generate_bindings(options: BindgenOptions, logger: Option<&Logger>, span: Span) -> Result<Vec<Gc<ast::Item>>, ()> {
    let l = DummyLogger;
    let logger = match logger {
        Some(l) => l,
        None => {
            &l as &Logger
        }
    };
    let globals = try!(parse_headers(&options, logger));
    Ok(gen::gen_mod(options.abi.as_slice(), options.links.as_slice(), globals, span))
}

struct DummyLogger;

#[allow(unused_variable)]
impl Logger for DummyLogger {
    fn error(&self, msg: &str) { }
    fn warn(&self, msg: &str) { }
}

fn parse_headers(options: &BindgenOptions, logger: &Logger) -> Result<Vec<Global>, ()> {
    let clang_opts = parser::ClangParserOptions {
        builtin_names: builtin_names(),
        builtins: options.builtins,
        match_pat: options.match_pat.clone(),
        emit_ast: options.emit_ast,
        fail_on_bitfield: options.fail_on_bitfield,
        fail_on_unknown_type: options.fail_on_unknown_type,
        clang_args: options.clang_args.clone(),
    };

    parser::parse(clang_opts, logger)
}

fn builtin_names() -> HashSet<String> {
    let mut names = HashSet::new();
    let keys = [
        "__va_list_tag",
        "__va_list",
    ];

    keys.iter().advance(|s| {
        names.insert(s.to_string());
        true
    });

    return names;
}
