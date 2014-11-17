#![feature(phase)]

#[phase(plugin)]
extern crate bindgen;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub mod ffi {
    bindgen!("math.h", link = "m")
}

fn main() {
    unsafe {
        assert_eq!(ffi::floor( 2.7),  2.0);
        assert_eq!(ffi::floor(-2.7), -3.0);
        assert_eq!(ffi::floor(-0.0),  0.0);
    }
}
