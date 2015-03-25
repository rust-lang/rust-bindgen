#![allow(dead_code)]

extern crate bindgen;
extern crate syntex_syntax as syntax;

mod support;

// Unused until we can generate code for tests
//mod test_cmath;
mod test_cxx;
mod test_enum;
mod test_decl;
mod test_extern;
mod test_func;
mod test_struct;
mod test_union;
mod test_builtins;
