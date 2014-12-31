#![feature(globs)]
#![feature(macro_rules)]
#![feature(phase)]
#![feature(quote)]

#[phase(plugin)]
extern crate bindgen;

extern crate bindgen;
extern crate libc;
extern crate syntax;

#[macro_escape]
mod support;

mod test_cmath;
mod test_decl;
mod test_func;
mod test_struct;
mod test_union;
mod test_builtins;
