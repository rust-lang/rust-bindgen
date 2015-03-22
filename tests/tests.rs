#![feature(quote, plugin)]
#![plugin(bindgen)]

extern crate bindgen;
extern crate libc;
extern crate syntax;

#[macro_use]
mod support;

mod test_cmath;
mod test_decl;
mod test_func;
mod test_struct;
mod test_union;
mod test_builtins;
