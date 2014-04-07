#![crate_type = "bin"]
#![feature(globs, managed_boxes, quote)]

extern crate collections;
extern crate syntax;
extern crate libc;

pub mod types;
pub mod clangll;
pub mod clang;
pub mod gen;
pub mod main;
