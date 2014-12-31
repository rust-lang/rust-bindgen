#![feature(phase)]

#[phase(plugin)]
extern crate bindgen;

extern crate libc;

#[test]
fn test_builtin_va_list() {
    mod ffi { bindgen!("headers/builtin_va_list.h", emit_builtins = true); }
    // Should test for more than compilation.
}

