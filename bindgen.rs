#![crate_type = "bin"]
#![feature(globs, managed_boxes, quote, phase)]

extern crate collections;
extern crate syntax;
extern crate libc;
#[phase(syntax, link)] extern crate log;

pub mod types;
pub mod clangll;
pub mod clang;
pub mod gen;
pub mod main;
