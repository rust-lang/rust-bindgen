use std::io::{stderr, Write};
use std::process::{Command, Stdio};

use bindgen;
use bindgen::{Logger, BindgenOptions};

use diff;

use syntax::ast;
use syntax::codemap;
use syntax::codemap::DUMMY_SP;
use syntax::parse;
use syntax::parse::token;
use syntax::print::pprust;
use syntax::ptr::P;

#[derive(Debug)]
struct TestLogger;

impl Logger for TestLogger {
    fn error(&self, msg: &str) {
        println!("err:  {}", msg);
    }

    fn warn(&self, msg: &str) {
        println!("warn: {}", msg);
    }
}

pub fn generate_bindings(mut options: BindgenOptions,
                         filename: &str)
                         -> Result<Vec<P<ast::Item>>, ()> {
    if filename.ends_with("hpp") {
        options.clang_args.push("-std=c++11".to_string());
        options.clang_args.push("-Wno-narrowing".to_string());
    }
    options.clang_args.push(filename.to_string());

    let logger = TestLogger;
    Ok(try!(bindgen::Bindings::generate(&options, Some(&logger as &Logger), None)).into_ast())
}

pub fn assert_bind_eq(options: BindgenOptions,
                      filename: &str,
                      reference_items_str: &str) {
    let ext_cx = mk_dummy_ext_ctxt();
    let generated_items =
        generate_bindings(options, &format!("tests/{}", filename)[..]).unwrap();

    let mut parser = parse::new_parser_from_source_str(ext_cx.parse_sess(), ext_cx.cfg(), "".to_string(), reference_items_str.to_string());
    let mut reference_items = Vec::new();
    while let Some(item) = parser.parse_item().unwrap() {
        reference_items.push(item);
    }

    // The ast::Items themselves have insignificant (for our purposes)
    // differences that make them difficult to compare directly.  So, compare
    // rendered versions, which is not beautiful, but should work.
    let reference_rendered = render_items(&reference_items);
    let generated_rendered = render_items(&generated_items);

    if reference_rendered != generated_rendered {
        println!("Generated bindings for {} do not match the reference bindings.", filename);
        println!("");
        println!("--- reference");
        println!("+++ generated");
        println!("");
        for diff in diff::lines(&reference_rendered, &generated_rendered) {
            match diff {
                diff::Result::Left(l)    => println!("- {}", l),
                diff::Result::Both(l, _) => println!("  {}", l),
                diff::Result::Right(r)   => println!("+ {}", r)
            }
        }
        panic!();
    }

    try_compile(&reference_rendered);
}

fn try_compile(src: &str) {
    let mut rustc = Command::new("rustc")
                        .arg("--crate-type=lib")
                        .arg("-Zno-trans")
                        .arg("-")
                        .stdin(Stdio::piped())
                        .stdout(Stdio::null())
                        .stderr(Stdio::piped())
                        .spawn()
                        .unwrap();
    rustc.stdin.as_mut().unwrap().write(src.as_bytes()).unwrap();
    let res = rustc.wait_with_output().unwrap();
    if !res.status.success() {
        stderr().write(&res.stderr).unwrap();
        panic!();
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
    DummyExtCtxt { sess: parse::ParseSess::new() }
}
