use std::cell::RefCell;
use std::default::Default;
use std::os;
use std::gc::Gc;

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
    let clang_args = visit.options.clang_args.connect(" ");
    visit.options.clang_args = parse_process_args(clang_args.as_slice());

    // Set the working dir to the directory containing the invoking rs file so
    // that clang searches for headers relative to it rather than the crate root
    let mod_dir = Vec::from_slice(Path::new(cx.codemap().span_to_filename(sp)).dirname());
    let cwd = os::getcwd();
    os::change_dir(&Path::new(mod_dir));
    
    // We want the span for errors to just match the bindgen! symbol
    // instead of the whole invocation which can span multiple lines
    let mut short_span = sp;
    short_span.hi = short_span.lo + codemap::BytePos(8);

    let logger = MacroLogger { sp: short_span, cx: cx };

    let ret = match generate_bindings(visit.options, Some(&logger as &Logger), short_span) {
        Ok(items) => {
            box BindgenResult { items: RefCell::new(Some(SmallVector::many(items))) } as Box<base::MacResult>
        }
        Err(_) => base::DummyResult::any(sp)
    };

    os::change_dir(&Path::new(cwd));

    ret
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

impl MacroArgsVisitor for BindgenArgsVisitor {
    fn visit_str(&mut self, mut name: Option<&str>, val: &str) -> bool {
        if name.is_some() { self.seen_named = true; }
        else if !self.seen_named { name = Some("clang_args") }
        match name {
            Some("link") => self.options.links.push((val.to_string(), None)),
            Some("link_static") => self.options.links.push((val.to_string(), Some("static".to_string()))),
            Some("link_framework") => self.options.links.push((val.to_string(), Some("framework".to_string()))),
            Some("abi") => self.options.abi = val.to_string(),
            Some("match") => self.options.match_pat.push(val.to_string()),
            Some("clang_args") => self.options.clang_args.push(val.to_string()),
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

// Parses macro invocations in the form [ident=|:]value where value is an ident or literal
// e.g. bindgen!(module_name, "header.h", emit_builtins=false, clang_args:"-I /usr/local/include")
fn parse_macro_opts(cx: &mut base::ExtCtxt, tts: &[ast::TokenTree], visit: &mut MacroArgsVisitor) -> bool {
    let mut parser = parse::new_parser_from_tts(cx.parse_sess(), cx.cfg(), Vec::from_slice(tts));
    let mut args_good = true;

    loop {
        let mut name: Option<String> = None;
        let mut span = parser.span;

        // Check for [ident=]value and if found save ident to name
        if parser.look_ahead(1, |t| t == &token::EQ) {
            match parser.bump_and_get() {
                token::IDENT(ident, _) => {
                    let ident = parser.id_to_interned_str(ident);
                    name = Some(ident.get().to_string());
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

// I'm sure there's a nicer way of doing it
fn as_str<'a>(owned: &'a Option<String>) -> Option<&'a str> {
    match owned {
        &Some(ref s) => Some(s.as_slice()),
        &None => None
    }
}

#[deriving(PartialEq, Eq)]
enum QuoteState {
    InNone,
    InSingleQuotes,
    InDoubleQuotes
}

fn parse_process_args(s: &str) -> Vec<String> {
    let s = s.trim();
    let mut parts = Vec::new();
    let mut quote_state = InNone;
    let mut positions = vec!(0);
    let mut last = ' ';
    for (i, c) in s.chars().chain(" ".chars()).enumerate() {
        match (last, c) {
            // Match \" set has_escaped and skip
            ('\\', '\"') => (),
            // Match \'
            ('\\', '\'') => (),
            // Match \<space>
            // Check we don't escape the final added space
            ('\\', ' ') if i < s.len() => (),
            // Match \\
            ('\\', '\\') => (),
            // Match <any>"
            (_, '\"') if quote_state == InNone => {
                quote_state = InDoubleQuotes;
                positions.push(i);
                positions.push(i + 1);
            },
            (_, '\"') if quote_state == InDoubleQuotes => {
                quote_state = InNone;
                positions.push(i);
                positions.push(i + 1);
            },
            // Match <any>'
            (_, '\'') if quote_state == InNone => {
                quote_state = InSingleQuotes;
                positions.push(i);
                positions.push(i + 1);
            },
            (_, '\'') if quote_state == InSingleQuotes => {
                quote_state = InNone;
                positions.push(i);
                positions.push(i + 1);
            },
            // Match <any><space>
            // If we are at the end of the string close any open quotes
            (_, ' ') if quote_state == InNone || i >= s.len() => {
                {
                    positions.push(i);

                    let starts = positions.iter().enumerate().filter(|&(i, _)| i % 2 == 0);
                    let ends = positions.iter().enumerate().filter(|&(i, _)| i % 2 == 1);

                    let part: Vec<String> = starts.zip(ends).map(|((_, start), (_, end))| s.slice(*start, *end).to_string()).collect();

                    let part = part.connect("");

                    if part.len() > 0 {
                        // Remove any extra whitespace outside the quotes
                        let part = part.as_slice().trim();
                        // Replace quoted characters
                        let part = part.replace("\\\"", "\"");
                        let part = part.replace("\\\'", "\'");
                        let part = part.replace("\\ ", " ");
                        let part = part.replace("\\\\", "\\");
                        parts.push(part);
                    }
                }

                positions.clear();
                positions.push(i + 1);
            },
            (_, _) => ()
        }
        last = c;
    }
    parts
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

struct BindgenResult {
    items: RefCell<Option<SmallVector<Gc<ast::Item>>>>
}

impl base::MacResult for BindgenResult {
    fn make_items(&self) -> Option<SmallVector<Gc<ast::Item>>> {
        self.items.borrow_mut().take()
    }
}

#[cfg(test)]
fn make_string_vec(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|p| p.to_string()).collect()
}

#[test]
fn test_parse_process_args() {
    assert_eq!(parse_process_args("a b c"), make_string_vec(["a", "b", "c"]));
    assert_eq!(parse_process_args("a \"b\" c"), make_string_vec(["a", "b", "c"]));
    assert_eq!(parse_process_args("a \'b\' c"), make_string_vec(["a", "b", "c"]));
    assert_eq!(parse_process_args("a \"b c\""), make_string_vec(["a", "b c"]));
    assert_eq!(parse_process_args("a \'\"b\"\' c"), make_string_vec(["a", "\"b\"", "c"]));
    assert_eq!(parse_process_args("a b\\ c"), make_string_vec(["a", "b c"]));
    assert_eq!(parse_process_args("a b c\\"), make_string_vec(["a", "b", "c\\"]));
}
