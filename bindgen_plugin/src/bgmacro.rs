use std::default::Default;
use std::env;
use std::path::Path;

use syntax::ast;
use syntax::codemap;
use syntax::ext::base;
use syntax::fold::Folder;
use syntax::parse;
use syntax::parse::token;
use syntax::ptr::P;
use syntax::util::small_vector::SmallVector;

use bindgen::{Bindings, BindgenOptions, LinkType, Logger, self};

pub fn bindgen_macro(cx: &mut base::ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<base::MacResult+'static> {
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
    visit.options.clang_args = parse_process_args(&clang_args);

    if let Some(path) = bindgen::get_include_dir() {
        visit.options.clang_args.push("-I".to_owned());
        visit.options.clang_args.push(path);
    }

    // Set the working dir to the directory containing the invoking rs file so
    // that clang searches for headers relative to it rather than the crate root
    let filename = cx.codemap().span_to_filename(sp);
    let mod_dir = Path::new(&filename).parent().unwrap();
    let cwd = match env::current_dir() {
      Ok(d)   => d,
      Err(e)  => panic!("Invalid current working directory: {}", e),
    };
    let p = Path::new(mod_dir);
    if let Err(e) = env::set_current_dir(&p) {
      panic!("Failed to change to directory {}: {}", p.display(), e);
    };

    // We want the span for errors to just match the bindgen! symbol
    // instead of the whole invocation which can span multiple lines
    let mut short_span = sp;
    short_span.hi = short_span.lo + codemap::BytePos(8);

    let logger = MacroLogger { sp: short_span, cx: cx };

    let ret = match Bindings::generate(&visit.options, Some(&logger as &Logger), None) {
        Ok(bindings) => {
            // syntex_syntax is not compatible with libsyntax so convert to string and reparse
            let bindings_str = bindings.to_string();
            // Unfortunately we lose span information due to reparsing
            let mut parser = parse::new_parser_from_source_str(cx.parse_sess(), cx.cfg(), "(Auto-generated bindings)".to_string(), bindings_str);

            let mut items = Vec::new();
            while let Ok(Some(item)) = parser.parse_item() {
                items.push(item);
            }

            Box::new(BindgenResult { items: Some(SmallVector::many(items)) }) as Box<base::MacResult>

        }
        Err(_) => base::DummyResult::any(sp)
    };

    let p = Path::new(&cwd);
    if let Err(e) = env::set_current_dir(&p) {
      panic!("Failed to return to directory {}: {}", p.display(), e);
    }

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
            Some("link") => self.options.links.push((val.to_string(), LinkType::Default)),
            Some("link_static") => self.options.links.push((val.to_string(), LinkType::Static)),
            Some("link_framework") => self.options.links.push((val.to_string(), LinkType::Framework)),
            Some("match") => self.options.match_pat.push(val.to_string()),
            Some("clang_args") => self.options.clang_args.push(val.to_string()),
            Some("enum_type") => self.options.override_enum_ty = val.to_string(),
            _ => return false
        }
        true
    }

    fn visit_int(&mut self, name: Option<&str>, _val: i64) -> bool {
        if name.is_some() { self.seen_named = true; }
        false
    }

    fn visit_bool(&mut self, name: Option<&str>, val: bool) -> bool {
        if name.is_some() { self.seen_named = true; }
        match name {
            Some("allow_unknown_types") => self.options.fail_on_unknown_type = !val,
            Some("emit_builtins") => self.options.builtins = val,
            _ => return false
        }
        true
    }

    fn visit_ident(&mut self, name: Option<&str>, _val: &str) -> bool {
        if name.is_some() { self.seen_named = true; }
        false
    }
}

// Parses macro invocations in the form [ident=|:]value where value is an ident or literal
// e.g. bindgen!(module_name, "header.h", emit_builtins=false, clang_args:"-I /usr/local/include")
fn parse_macro_opts(cx: &mut base::ExtCtxt, tts: &[ast::TokenTree], visit: &mut MacroArgsVisitor) -> bool {
    let mut parser = cx.new_parser_from_tts(tts);
    let mut args_good = true;

    loop {
        let mut name: Option<String> = None;
        let mut span = parser.span;

        // Check for [ident=]value and if found save ident to name
        if parser.look_ahead(1, |t| t == &token::Eq) {
            match parser.bump_and_get() {
                Ok(token::Ident(ident, _)) => {
                    let ident = parser.id_to_interned_str(ident);
                    name = Some(ident.to_string());
                    if let Err(_) = parser.expect(&token::Eq) {
                        return false;
                    }
                },
                _ => {
                    cx.span_err(span, "invalid argument format");
                    return false
                }
            }
        }

        match parser.token {
            // Match [ident]
            token::Ident(val, _) => {
                let val = parser.id_to_interned_str(val);
                span.hi = parser.span.hi;
                if let Err(_) = parser.bump() {
                    return false;
                }

                // Bools are simply encoded as idents
                let ret = match &*val {
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
                let expr = cx.expander().fold_expr(parser.parse_expr().unwrap());
                span.hi = expr.span.hi;
                match expr.node {
                    ast::ExprLit(ref lit) => {
                        let ret = match lit.node {
                            ast::LitStr(ref s, _) => visit.visit_str(as_str(&name), &*s),
                            ast::LitBool(b) => visit.visit_bool(as_str(&name), b),
                            ast::LitInt(i, ast::SignedIntLit(_, sign)) |
                            ast::LitInt(i, ast::UnsuffixedIntLit(sign)) => {
                                let i = i as i64;
                                let i = if sign == ast::Minus { -i } else { i };
                                visit.visit_int(as_str(&name), i)
                            },
                            ast::LitInt(i, ast::UnsignedIntLit(_)) => visit.visit_int(as_str(&name), i as i64),
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

        if parser.check(&token::Eof) {
            return args_good
        }

        if parser.eat(&token::Comma).is_err() {
            cx.span_err(parser.span, "invalid argument format");
            return false
        }
    }
}

// I'm sure there's a nicer way of doing it
fn as_str<'a>(owned: &'a Option<String>) -> Option<&'a str> {
    match owned {
        &Some(ref s) => Some(s),
        &None => None
    }
}

#[derive(PartialEq, Eq)]
enum QuoteState {
    InNone,
    InSingleQuotes,
    InDoubleQuotes
}

fn parse_process_args(s: &str) -> Vec<String> {
    let s = s.trim();
    let mut parts = Vec::new();
    let mut quote_state = QuoteState::InNone;
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
            (_, '\"') if quote_state == QuoteState::InNone => {
                quote_state = QuoteState::InDoubleQuotes;
                positions.push(i);
                positions.push(i + 1);
            },
            (_, '\"') if quote_state == QuoteState::InDoubleQuotes => {
                quote_state = QuoteState::InNone;
                positions.push(i);
                positions.push(i + 1);
            },
            // Match <any>'
            (_, '\'') if quote_state == QuoteState::InNone => {
                quote_state = QuoteState::InSingleQuotes;
                positions.push(i);
                positions.push(i + 1);
            },
            (_, '\'') if quote_state == QuoteState::InSingleQuotes => {
                quote_state = QuoteState::InNone;
                positions.push(i);
                positions.push(i + 1);
            },
            // Match <any><space>
            // If we are at the end of the string close any open quotes
            (_, ' ') if quote_state == QuoteState::InNone || i >= s.len() => {
                {
                    positions.push(i);

                    let starts = positions.iter().enumerate().filter(|&(i, _)| i % 2 == 0);
                    let ends = positions.iter().enumerate().filter(|&(i, _)| i % 2 == 1);

                    let part: Vec<String> = starts.zip(ends).map(|((_, start), (_, end))| s[*start..*end].to_string()).collect();

                    let part = part.join("");

                    if part.len() > 0 {
                        // Remove any extra whitespace outside the quotes
                        let part = part.trim();
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

struct MacroLogger<'a, 'b:'a> {
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
    items: Option<SmallVector<P<ast::Item>>>
}

impl base::MacResult for BindgenResult {
    fn make_items(mut self: Box<BindgenResult>) -> Option<SmallVector<P<ast::Item>>> {
        self.items.take()
    }
}

#[test]
fn test_parse_process_args() {
    assert_eq!(parse_process_args("a b c"),         vec!("a", "b", "c"));
    assert_eq!(parse_process_args("a \"b\" c"),     vec!("a", "b", "c"));
    assert_eq!(parse_process_args("a \'b\' c"),     vec!("a", "b", "c"));
    assert_eq!(parse_process_args("a \"b c\""),     vec!("a", "b c"));
    assert_eq!(parse_process_args("a \'\"b\"\' c"), vec!("a", "\"b\"", "c"));
    assert_eq!(parse_process_args("a b\\ c"),       vec!("a", "b c"));
    assert_eq!(parse_process_args("a b c\\"),       vec!("a", "b", "c\\"));
}
