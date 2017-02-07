//! Intermediate representation for attributes.
use std::str;

use clang::Cursor;
use clang_sys::{CXCursor_UnexposedAttr, CXChildVisit_Continue};
use cexpr::token::{Token, Kind};

use super::context::BindgenContext;

/// The special attribute
#[derive(Clone, Debug)]
pub enum Attribute {
    /// This attribute results in a warning if the type is used anywhere in the source file.
    Deprecated(Option<String>),
    /// This attribute means that variables of that type are meant to appear possibly unused.
    Unused,
    /// This attribute attached to a function, means that code must be emitted for the function 
    /// even if it appears that the function is not referenced.
    Used,
    /// This attribute on functions is used to inform the compiler that the function is unlikely to be executed.
    Cold,
    /// Many functions do not examine any values except their arguments, 
    /// and have no effects except the return value.
    Const,
    /// This attribute causes the function to be called automatically before execution enters main ().
    Constructor(Option<isize>),
    /// This attribute causes the function to be called automatically after main () completes or exit () is called.
    Destructor(Option<isize>),
    /// This attribute specifies a minimum alignment (in bytes)
    Aligned(Vec<Token>),
    /// An attribute whose specific kind is not exposed via this interface.
    Unexposed(String, Vec<Token>)
}

impl Attribute {
    /// Construct a new `Attribute`.
    pub fn new(tokens: Vec<Token>) -> Self {
        // https://gcc.gnu.org/onlinedocs/gcc/Attribute-Syntax.html#Attribute-Syntax
        assert!(!tokens.is_empty());

        let (token, args) = tokens.split_first().unwrap();

        assert_eq!(token.kind, Kind::Identifier);

        let name = unsafe { str::from_utf8_unchecked(&token.raw) };

        debug!("__attribute__(({}({:?})))", name, args);

        match name {
            "deprecated" => {
                let text = if args.len() == 3 &&
                                  args[0].kind == Kind::Punctuation && args[0].raw.as_ref() == b"(" &&
                                  args[1].kind == Kind::Literal &&
                                  args[2].kind == Kind::Punctuation && args[2].raw.as_ref() == b")" {
                    str::from_utf8(args[1].raw.as_ref()).ok().map(String::from)
                } else {
                    None
                };

                Attribute::Deprecated(text)
            }
            "unused" => Attribute::Unused,
            "used" => Attribute::Used,
            "cold" => Attribute::Cold,
            "const" => Attribute::Const,
            "constructor" => {
                let priority = if args.len() == 3 &&
                                  args[0].kind == Kind::Punctuation && args[0].raw.as_ref() == b"(" &&
                                  args[1].kind == Kind::Literal &&
                                  args[2].kind == Kind::Punctuation && args[2].raw.as_ref() == b")" {
                    str::from_utf8(args[1].raw.as_ref()).ok().and_then(|s| s.parse::<isize>().ok())
                } else {
                    None
                };

                Attribute::Constructor(priority)
            }
            "destructor" => {
                let priority = if args.len() == 3 &&
                                  args[0].kind == Kind::Punctuation && args[0].raw.as_ref() == b"(" &&
                                  args[1].kind == Kind::Literal &&
                                  args[2].kind == Kind::Punctuation && args[2].raw.as_ref() == b")" {
                    str::from_utf8(args[1].raw.as_ref()).ok().and_then(|s| s.parse::<isize>().ok())
                } else {
                    None
                };

                Attribute::Destructor(priority)
            }
            "aligned" => Attribute::Aligned(Vec::from(args)),
            _ => Attribute::Unexposed(String::from(name), Vec::from(args)),
        }
    }

    /// Parse a `Cursor` for `Vec<Attribute>`.
    pub fn parse(cur: &Cursor, ctx: &BindgenContext) -> Vec<Self> {
        let mut attributes = vec![];

        if let Some(tokens) = ctx.translation_unit().cexpr_tokens(&cur) {
            let mut c = 0;
            let mut iter = tokens.iter();

            while c >= 0 {
                let tokens = iter.by_ref().take_while(|ref token| {
                    if token.kind == Kind::Punctuation {
                        c += match token.raw.as_ref() {
                                b"(" => 1,
                                b")" => -1,
                                b"," if c == 0 => return false,
                                _ => 0,
                            }
                    }

                    c >= 0
                }).map(|token| token.clone()).collect::<Vec<Token>>();

                if tokens.is_empty() {
                    break
                } else {
                    attributes.push(Attribute::new(tokens));
                }
            }
        }

        attributes
    }

    /// Extract `Vec<Attribute>` from cursor's children.
    pub fn extract(cur: &Cursor, ctx: &BindgenContext) -> Vec<Self> {
        let mut attributes = vec![];

        cur.visit(|cur| {
            match cur.kind() {
                CXCursor_UnexposedAttr => {
                    attributes.append(&mut Attribute::parse(&cur, ctx))
                }
                _ => {}
            }
            CXChildVisit_Continue
        });

        attributes
    }

    /// Whether this attribute whose specific kind is not exposed.
    pub fn is_unexposed(&self) -> bool {
        match *self {
            Attribute::Unexposed(..) |
            Attribute::Aligned(..) => true,
            _ => false,
        }
    }
}