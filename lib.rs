#![crate_id = "bindgen"]
#![crate_type = "dylib"]
#![feature(globs, managed_boxes, quote, phase, macro_registrar)]

extern crate collections;
extern crate syntax;
extern crate libc;
#[phase(syntax, link)] extern crate log;

use collections::HashSet;
use std::default::Default;
use syntax::ast;
use syntax::ext::base;
use syntax::parse::token;

use types::Global;

mod types;
mod clangll;
mod clang;
mod gen;
mod parser;
mod macro;

#[macro_registrar]
pub fn macro_registrar(register: |ast::Name, base::SyntaxExtension|) {
    let expander = box base::BasicMacroExpander { expander: macro::bindgen_macro, span: None };
    register(token::intern("bindgen"), base::NormalTT(expander, None))
}

pub struct BindgenOptions {
    pub match_pat: Vec<~str>,
    pub abi: ~str,
    pub builtins: bool,
    pub links: Vec<(~str, Option<~str>)>,
    pub emit_ast: bool,
    pub fail_on_bitfield: bool,
    pub fail_on_unknown_type: bool,
    pub clang_args: Vec<~str>,
}

impl Default for BindgenOptions {
    fn default() -> BindgenOptions {
        BindgenOptions {
            match_pat: Vec::new(),
            abi: "C".to_owned(),
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

pub fn generate_bindings(options: BindgenOptions, logger: Option<&Logger>) -> Result<Vec<@ast::Item>, ()> {
    let l = DummyLogger;
    let logger = match logger {
        Some(l) => l,
        None => {
            &l as &Logger
        }
    };
    let globals = try!(parse_headers(&options, logger));
    Ok(gen::gen_mod(options.abi.as_slice(), options.links.as_slice(), globals))
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

fn builtin_names() -> HashSet<~str> {
    let mut names = HashSet::new();
    let keys = ~[
        "__va_list_tag".to_owned(),
        "__va_list".to_owned(),
    ];

    keys.move_iter().advance(|s| {
        names.insert(s);
        true
    });

    return names;
}