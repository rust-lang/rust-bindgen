#![feature(phase)]

#[phase(plugin)]
extern crate bindgen;

extern crate libc;

#[test]
fn test_unnamed_bitfields() {
    mod ffi { bindgen!("headers/unnamed_bitfields.h"); }
    // Check that there are no unnamed struct fields
}