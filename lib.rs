#![crate_id = "bindgen"]
#![crate_type = "dylib"]
#![feature(globs, managed_boxes, quote, phase)]

extern crate collections;
extern crate syntax;
extern crate libc;
#[phase(syntax, link)] extern crate log;

pub use main::main;

mod types;
mod clangll;
mod clang;
mod gen;
mod main;
mod parser;