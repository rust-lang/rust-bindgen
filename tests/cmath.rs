#![feature(phase)]

#[phase(plugin)]
extern crate bindgen;

extern crate libc;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub mod ffi {
    bindgen!("/usr/include/math.h", link = "m");
}

#[test]
fn test_floor_is_bound_and_callable() {
    unsafe {
        assert_eq!(ffi::floor( 2.7),  2.0);
        assert_eq!(ffi::floor(-2.7), -3.0);
        assert_eq!(ffi::floor(-0.0),  0.0);
    }
}
