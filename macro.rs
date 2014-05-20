use std::cell::RefCell;
use std::default::Default;
use std::os;
use syntax::ast;
use syntax::codemap;
use syntax::ext::base;
use syntax::parse;
use syntax::parse::token;
use syntax::util::small_vector::SmallVector;

use super::{generate_bindings, BindgenOptions, Logger};

pub fn bindgen_macro(cx: &mut base::ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<base::MacResult> {
    let mut visit = BindgenArgsVisitor {
        options: Default::default(),
        seen_named: false
    };

    visit.options.builtins = true;
    if !parse_macro_opts(cx, tts, &mut visit) {
        return base::DummyResult::any(sp);
    }

    // Reparse clang_args as it is passed in string form
    let clang_args = visit.options.clang_args.iter().fold("".to_owned(), |a, s| format!("{} {}", a, s));
    visit.options.clang_args = parse_process_args(clang_args.as_slice());

    // Set the working dir to the directory containing the invoking rs file so
    // that clang searches for headers relative to it rather than the crate root
    let mod_dir = Vec::from_slice(Path::new(cx.codemap().span_to_filename(sp)).dirname());
    let cwd = os::getcwd();
    os::change_dir(&Path::new(mod_dir));
    
    // We want the span for the logger to just match the bindgen! symbol
    // instead of the whole invocation which can span multiple lines
    let mut log_span = sp;
    log_span.hi = log_span.lo + codemap::BytePos(8);
    let logger = MacroLogger { sp: log_span, cx: cx };

    let ret = match generate_bindings(visit.options, Some(&logger as &Logger)) {
        Ok(items) => {
            box BindgenResult { items: RefCell::new(Some(SmallVector::many(items))) } as Box<base::MacResult>
        }
        Err(_) => base::DummyResult::any(sp)
    };

    os::change_dir(&Path::new(cwd));

    ret
}

struct BindgenResult {
    items: RefCell<Option<SmallVector<@ast::Item>>>
}

impl base::MacResult for BindgenResult {
    fn make_items(&self) -> Option<SmallVector<@ast::Item>> {
        self.items.borrow_mut().take()
    }
}

struct MacroLogger<'a, 'b> {
    sp: codemap::Span,
    cx: &'a base::ExtCtxt<'b>
}

impl<'a, 'b> Logger for MacroLogger<'a, 'b> {
    fn error(&self, msg: &str) {
        self.cx.span_err(self.sp, msg)
    }

    fn warn(&self, msg: &str) {
        self.cx.span_warn(self.sp, msg)
    }
}

trait MacroArgsVisitor {
    fn visit_str(&mut self, name: Option<&str>, val: &str) -> bool;
    fn visit_int(&mut self, name: Option<&str>, val: i64) -> bool;
    fn visit_bool(&mut self, name: Option<&str>, val: bool) -> bool;
    fn visit_ident(&mut self, name: Option<&str>, ident: &str) -> bool;
}

struct BindgenArgsVisitor {
    pub options: BindgenOptions,
    seen_named: bool
}

fn parse_process_args(s: &str) -> Vec<~str> {
    // 1: Parse into blocks
    // 2: For blocks with quoted quotes remove those quoted quotes
    let mut parts = Vec::new();
    let mut in_quotes = false;
    let mut has_escaped_quotes = false;
    let mut start_idx = 0;
    let mut last = ' ';
    for (i, c) in s.chars().chain(" ".chars()).enumerate() {
        match (last, c) {
            // Match \" set has_escaped and skip
            ('\\', '\"') => has_escaped_quotes = true,
            // Match <any>"
            (_, '\"') => in_quotes = !in_quotes,
            // Match <any><space>
            (_, ' ') => {
                if !in_quotes {
                    let mut part = s.slice(start_idx, i).to_owned();
                    if has_escaped_quotes {
                        part = part.replace("\\\"", "\"");
                        has_escaped_quotes = false;
                    }
                    if part.len() > 0 {
                        parts.push(part);
                    }
                    start_idx = i + 1;
                }
            },
            (_, _) => ()
        }
        last = c;
    }
    parts
}

impl MacroArgsVisitor for BindgenArgsVisitor {
    fn visit_str(&mut self, mut name: Option<&str>, val: &str) -> bool {
        if name.is_some() { self.seen_named = true; }
        else if !self.seen_named { name = Some("clang_args") }
        match name {
            Some("link") => self.options.links.push((val.to_owned(), None)),
            Some("link_static") => self.options.links.push((val.to_owned(), Some("static".to_owned()))),
            Some("link_framework") => self.options.links.push((val.to_owned(), Some("framework".to_owned()))),
            Some("abi") => self.options.abi = val.to_owned(),
            Some("match") => self.options.match_pat.push(val.to_owned()),
            Some("clang_args") => self.options.clang_args.push(val.to_owned()),
            _ => return false
        }
        true
    }

    #[allow(unused_variable)]
    fn visit_int(&mut self, name: Option<&str>, val: i64) -> bool {
        if name.is_some() { self.seen_named = true; }
        false
    }

    fn visit_bool(&mut self, name: Option<&str>, val: bool) -> bool {
        if name.is_some() { self.seen_named = true; }
        match name {
            Some("allow_bitfields") => self.options.fail_on_bitfield = !val,
            Some("allow_unknown_types") => self.options.fail_on_unknown_type = !val,
            Some("emit_builtins") => self.options.builtins = val,
            _ => return false
        }
        true
    }

    #[allow(unused_variable)]
    fn visit_ident(&mut self, name: Option<&str>, val: &str) -> bool {
        if name.is_some() { self.seen_named = true; }
        false
    }
}

// I'm sure there's a nicer way of doing it
fn as_str<'a>(owned: &'a Option<~str>) -> Option<&'a str> {
    match owned {
        &Some(ref s) => Some(s.as_slice()),
        &None => None
    }
}

// Parses macro invocations in the form [ident=|:]value where value is an ident or literal
// e.g. bindgen!(module_name, "header.h", emit_builtins=false, clang_args:"-I /usr/local/include")
fn parse_macro_opts(cx: &mut base::ExtCtxt, tts: &[ast::TokenTree], visit: &mut MacroArgsVisitor) -> bool {
    let mut parser = parse::new_parser_from_tts(cx.parse_sess(), cx.cfg(), Vec::from_slice(tts));
    let mut args_good = true;

    loop {
        let mut name: Option<~str> = None;
        let mut span = parser.span;

        // Check for [ident=]value and if found save ident to name
        if parser.look_ahead(1, |t| t == &token::EQ) {
            match parser.bump_and_get() {
                token::IDENT(ident, _) => {
                    let ident = parser.id_to_interned_str(ident);
                    name = Some(ident.get().to_owned());
                    parser.expect(&token::EQ);
                },
                _ => {
                    cx.span_err(span, "invalid argument format");
                    return false
                }
            }
        }

        match parser.token {
            // Match [ident]
            token::IDENT(val, _) => {
                let val = parser.id_to_interned_str(val);
                span.hi = parser.span.hi;
                parser.bump();
                
                // Bools are simply encoded as idents
                let ret = match val.get() {
                    "true" => visit.visit_bool(as_str(&name), true),
                    "false" => visit.visit_bool(as_str(&name), false),
                    val => visit.visit_ident(as_str(&name), val)
                };
                if !ret {
                    cx.span_err(span, "invalid argument");
                    args_good = false;
                }
            }
            // Match [literal] and parse as an expression so we can expand macros
            _ => {
                let expr = cx.expand_expr(parser.parse_expr());
                span.hi = expr.span.hi;
                match expr.node {
                    ast::ExprLit(lit) => {
                        let ret = match lit.node {
                            ast::LitStr(ref s, _) => visit.visit_str(as_str(&name), s.get()),
                            ast::LitBool(b) => visit.visit_bool(as_str(&name), b),
                            ast::LitIntUnsuffixed(i) |
                            ast::LitInt(i, _) => visit.visit_int(as_str(&name), i),
                            ast::LitUint(i, _) => visit.visit_int(as_str(&name), i as i64),
                            _ => {
                                cx.span_err(span, "invalid argument format");
                                return false
                            }
                        };
                        if !ret {
                            cx.span_err(span, "invalid argument");
                            args_good = false;
                        }
                    },
                    _ => {
                        cx.span_err(span, "invalid argument format");
                        return false
                    }
                }
            }
        }

        if parser.eat(&token::EOF) {
            return args_good
        }

        if !parser.eat(&token::COMMA) {
            cx.span_err(parser.span, "invalid argument format");
            return false
        }
    }
}

