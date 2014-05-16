use std::default::Default;
use std::os;
use syntax::ast;
use syntax::codemap;
use syntax::codemap::DUMMY_SP;
use syntax::ext::base;
use syntax::parse;
use syntax::parse::token;
use syntax::print::pprust;

use super::{generate_bindings, BindgenOptions, Logger};

pub fn bindgen_macro(cx: &mut base::ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<base::MacResult> {
    let mut visit = BindgenArgsVisitor::new();
    visit.options.builtins = true;
    if !parse_macro_opts(cx, tts, &mut visit) {
        return base::DummyResult::any(sp);
    }
    let mod_name = visit.mod_name.clone();

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
        Ok((module, attrs)) => {
            let item = @ast::Item {
                ident: cx.ident_of(mod_name),
                attrs: attrs,
                id: ast::DUMMY_NODE_ID,
                node: ast::ItemMod(module),
                vis: ast::Inherited,
                span: DUMMY_SP
            };
            base::MacItem::new(item)
        }
        Err(_) => base::DummyResult::any(sp)
    };

    os::change_dir(&Path::new(cwd));

    ret
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
    pub mod_name: ~str,
    pub options: BindgenOptions,
    idx: int,
    named: bool
}

impl BindgenArgsVisitor {
    fn new() -> BindgenArgsVisitor {
        BindgenArgsVisitor {
            mod_name: "bindings".to_owned(),
            options: Default::default(),
            idx: 0
        }
    }
}

impl MacroArgsVisitor for BindgenArgsVisitor {
    fn visit_str(&mut self, mut name: Option<&str>, val: &str) -> bool {
        self.idx += 1;
        if self.idx > 1 && name.is_none() {
            name = Some("header")
        }
        match name {
            Some("link") => self.options.links.push(val.to_owned()),
            Some("abi") => self.options.abi = val.to_owned(),
            Some("match") => self.options.match_pat.push(val.to_owned()),
            Some("header") | Some("clang_args") => self.options.clang_args.push(val.to_owned()),
            _ => return false
        }
        true
    }

    #[allow(unused_variable)]
    fn visit_int(&mut self, name: Option<&str>, val: i64) -> bool {
        self.idx += 1;
        false
    }

    fn visit_bool(&mut self, name: Option<&str>, val: bool) -> bool {
        self.idx += 1;
        match name {
            Some("allow_bitfields") => self.options.fail_on_bitfield = !val,
            Some("allow_unknown_types") => self.options.fail_on_unknown_type = !val,
            Some("emit_builtins") => self.options.builtins = val,
            _ => return false
        }
        true
    }

    fn visit_ident(&mut self, name: Option<&str>, val: &str) -> bool {
        self.idx += 1;
        if self.idx == 1 {
            self.mod_name = val.to_owned();
            return true
        }
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
        if parser.look_ahead(1, |t| t == &token::EQ || t == &token::COLON) {
            match parser.bump_and_get() {
                token::IDENT(ident, _) => {
                    let ident = parser.id_to_interned_str(ident);
                    name = Some(ident.get().to_owned());
                    parser.expect_one_of(&[token::EQ, token::COLON], &[]);
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

