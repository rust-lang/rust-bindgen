#![feature(plugin)]
#![plugin(bindgen_plugin)]
#![allow(dead_code)]

extern crate bindgen;
extern crate libc;
extern crate syntex_syntax as syntax;

mod support;

mod test_cmath;
mod test_decl;
mod test_func;
mod test_struct;
mod test_union;
mod test_builtins;
