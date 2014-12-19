extern crate bindgen;
extern crate regex;
extern crate syntax;

use bindgen::{Logger, BindgenOptions};
use regex::Regex;
use std::default::Default;
use syntax::ast;
use syntax::codemap::DUMMY_SP;
use syntax::print::pp;
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
    bindgen::generate_bindings(options, Some(&logger as &Logger), DUMMY_SP)
}

pub fn generate_unpretty_output(filename: &str) -> String {
    let output = pprust::to_string(|s| {
        // HACK: Replace the default printer with a very wide printer.  This
        //       makes it easier to reason about how the unpretty'd output
        //       will look, at the cost of tying us to the implementation
        //       details of pprust::to_string.
        s.s = pp::mk_printer(box Vec::new(), 4096);

        let items = generate_bindings(filename).unwrap();

        let module = ast::Mod {
            inner: DUMMY_SP,
            view_items: Vec::new(),
            items: items,
        };
        s.print_mod(&module, &[])
    });

    unpretty(output.as_slice())
}

pub fn unpretty(s: &str) -> String {
    let re = Regex::new(r"\s+").unwrap();
    re.replace_all(s, " ")
}
