use bindgen;
use bindgen::{Logger, BindgenOptions};

use std::default::Default;

use syntax::ast;
use syntax::codemap;
use syntax::codemap::{Span, DUMMY_SP};
use syntax::parse;
use syntax::parse::token;
use syntax::print::pprust;
use syntax::ptr::P;

struct TestLogger;

impl Logger for TestLogger {
    fn error(&self, msg: &str) {
        println!("err:  {}", msg);
    }

    fn warn(&self, msg: &str) {
        println!("warn: {}", msg);
    }
}

pub fn generate_bindings(filename: &str) -> Result<Vec<P<ast::Item>>, ()> {
    let mut options:BindgenOptions = Default::default();
    options.clang_args.push(filename.to_string());

    let logger = TestLogger;
    Ok(try!(bindgen::Bindings::generate(&options, Some(&logger as &Logger), None)).into_ast())
}

pub fn test_bind_eq<F>(filename: &str, f:F)
    where F: FnOnce(DummyExtCtxt) -> Vec<Option<P<ast::Item>>>
{
    let ext_cx = mk_dummy_ext_ctxt();
    let items = generate_bindings(filename).unwrap();
    let quoted =f(ext_cx).into_iter().map(|x| x.unwrap()).collect();

    // The ast::Items themselves have insignificant (for our purposes)
    // differences that make them difficult to compare directly.  So, compare
    // rendered versions, which is not beautiful, but should work.
    assert_eq!(render_items(&quoted), render_items(&items));
}

macro_rules! assert_bind_eq {
    ($filename:expr, $ext_cx:ident, $($quote:expr),*) => {
        ::support::test_bind_eq(concat!("tests/", $filename), |ext_cx| {
            let $ext_cx = &ext_cx;
            vec!($($quote),*)
        });
    }
}

fn render_items(items: &Vec<P<ast::Item>>) -> String {
    pprust::to_string(|s| {
        let module = ast::Mod {
            inner: DUMMY_SP,
            items: items.clone(),
        };
        s.print_mod(&module, &[])
    })
}

pub struct DummyExtCtxt {
    sess: parse::ParseSess,
}

impl DummyExtCtxt {
    pub fn cfg(&self) -> ast::CrateConfig {
        vec!()
    }
    pub fn parse_sess(&self) -> &parse::ParseSess {
        &self.sess
    }
    pub fn call_site(&self) -> codemap::Span {
        codemap::Span {
            lo: codemap::BytePos(0),
            hi: codemap::BytePos(0),
            expn_id: codemap::NO_EXPANSION
        }
    }
    pub fn ident_of(&self, s: &str) -> ast::Ident {
        token::str_to_ident(s)
    }
    pub fn name_of(&self, s: &str) -> ast::Name {
        token::intern(s)
    }
}

fn mk_dummy_ext_ctxt<'a>() -> DummyExtCtxt {
    DummyExtCtxt { sess: parse::new_parse_sess() }
}
