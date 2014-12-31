use bindgen;
use bindgen::{Logger, BindgenOptions};
use std::default::Default;
use std::fmt;
use syntax::ast;
use syntax::print::pprust;
use syntax::ptr::P;

use syntax::codemap;
use syntax::codemap::{Span, DUMMY_SP};
use syntax::parse;
use syntax::parse::token;

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

pub fn test_bind_eq(filename: &str, f:|ext_cx: DummyExtCtxt| -> Vec<Option<P<ast::Item>>>) {
    let ext_cx = mk_dummy_ext_ctxt();
    let items = normalize_attr_ids(generate_bindings(filename).unwrap());
    let quoted = normalize_attr_ids(f(ext_cx).into_iter().map(|x| x.unwrap()).collect());
    assert_eq!(PrettyItems { items: items },
               PrettyItems { items: quoted });
}

macro_rules! assert_bind_eq {
    ($filename:expr, $ext_cx:ident, $($quote:expr),*) => {
        ::support::test_bind_eq(concat!("tests/", $filename), |ext_cx| {
            let $ext_cx = &ext_cx;
            vec!($($quote),*)
        });
    }
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

#[deriving(PartialEq)]
struct PrettyItems {
    pub items: Vec<P<ast::Item>>,
}

impl fmt::Show for PrettyItems {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = pprust::to_string(|s| {
            let module = ast::Mod {
                inner: DUMMY_SP,
                view_items: Vec::new(),
                items: self.items.clone(),
            };
            s.print_mod(&module, &[])
        });
        write!(f, "\n{}\n", output)
    }
}

// libsyntax uses a thread-local variable to create unique AttrId values during
// parsing and quoting.  normalize_attr_ids makes sure that all AttrId values
// for the passed `items` start at zero and proceed upward.  This is necessary
// to correctly compare otherwise-identical ASTs.
fn normalize_attr_ids(items: Vec<P<ast::Item>>) -> Vec<P<ast::Item>> {
    use std::mem;
    use syntax::visit::*;

    struct Vis<'a> {
        attr_id: uint,
        items: Vec<P<ast::Item>>,
    }

    impl<'a> Vis<'a> {
        // TODO: visit_crate?  ast::visit::Visitor does not deal with them directly,
        //       but concievably, it could eventually come up.

        fn rewrite_attrs(&mut self, attrs: Vec<ast::Attribute>) -> Vec<ast::Attribute> {
            attrs.into_iter().map(|mut attr| {
                attr.node.id = ast::AttrId(self.attr_id);
                self.attr_id += 1;
                attr
            }).collect()
        }
    }

    unsafe fn force_mutable<T>(x: &T) -> &mut T {
        mem::transmute(x)
    }

    macro_rules! rewrite_attrs {
        ($self_:ident, $item:expr) => {
            unsafe {
                let unsafe_item = force_mutable($item);
                let new_attrs = mem::replace(&mut unsafe_item.attrs, vec!());
                let new_attrs = $self_.rewrite_attrs(new_attrs);
                mem::replace(&mut unsafe_item.attrs, new_attrs);
            }
        }
    }

    impl<'a, 'v> Visitor<'v> for Vis<'a> {
        fn visit_item(&mut self, i: &ast::Item) {
            rewrite_attrs!(self, i);

            match i.node {
                ast::ItemImpl(_, _, _, _, ref impl_items) => {
                    for impl_item in impl_items.iter() {
                        match *impl_item {
                            ast::ImplItem::MethodImplItem(_) => { }
                            ast::ImplItem::TypeImplItem(ref typedef) => {
                                rewrite_attrs!(self, typedef.deref());
                            }
                        }
                    }
                }
                _ => { }
            }

            walk_item(self, i);
        }

        fn visit_foreign_item(&mut self, i: &ast::ForeignItem) {
            rewrite_attrs!(self, i);
            walk_foreign_item(self, i);
        }

        fn visit_fn(&mut self, fk: FnKind, fd: &ast::FnDecl, b: &ast::Block, s: Span, _: ast::NodeId) {
            match fk {
                FkItemFn(_, _, _, _) | FkFnBlock => { }
                FkMethod(_, _, method) => {
                    rewrite_attrs!(self, method);
                }
            }
            walk_fn(self, fk, fd, b, s);
        }

        fn visit_arm(&mut self, a: &ast::Arm) {
            rewrite_attrs!(self, a);
            walk_arm(self, a);
        }

        fn visit_ty_method(&mut self, t: &ast::TypeMethod) {
            rewrite_attrs!(self, t);
            walk_ty_method(self, t);
        }

        fn visit_variant(&mut self, v: &ast::Variant, g: &ast::Generics) {
            rewrite_attrs!(self, &v.node);
            walk_variant(self, v, g);
        }

        fn visit_view_item(&mut self, i: &ast::ViewItem) {
            rewrite_attrs!(self, i);
            walk_view_item(self, i);
        }

        fn visit_struct_field(&mut self, s: &ast::StructField) {
            rewrite_attrs!(self, &s.node);
            walk_struct_field(self, s);
        }

        fn visit_trait_item(&mut self, t: &ast::TraitItem) {
            match *t {
                ast::TraitItem::RequiredMethod(_) |
                ast::TraitItem::ProvidedMethod(_) => { }
                ast::TraitItem::TypeTraitItem(ref assoc_ty) => {
                    rewrite_attrs!(self, assoc_ty.deref());
                }
            }
            walk_trait_item(self, t);
        }
    }

    let mut visitor = Vis { attr_id: 0, items: vec!() };
    for item in items.into_iter() {
        visitor.visit_item(item.deref());
        visitor.items.push(item);
    }
    visitor.items
}
