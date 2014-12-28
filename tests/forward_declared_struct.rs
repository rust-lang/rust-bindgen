#![feature(phase)]

#[phase(plugin)]
extern crate bindgen;

extern crate libc;

#[test]
fn test_struct_containing_forward_declared_struct() {
    mod ffi { bindgen!("headers/struct_containing_forward_declared_struct.h"); }
    // Checking that struct b is not duplicated
}