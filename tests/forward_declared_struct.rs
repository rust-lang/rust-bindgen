#![feature(phase)]

#[phase(plugin)]
extern crate bindgen;

extern crate libc;

#[test]
fn test_struct_containing_forward_declared_struct() {
    mod ffi { bindgen!("headers/struct_containing_forward_declared_struct.h"); }
    // Check that struct b is not duplicated
}

#[test]
fn test_forward_declared_struct() {
	mod ffi { bindgen!("headers/forward_declared_struct.h"); }

	let a = ffi::Struct_a { b: 1 };
	let c = ffi::Struct_c { d: 1 };
	
	a.b;
	c.d;
}