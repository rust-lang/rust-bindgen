#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug)]
pub struct foo {
    _unused: [u8; 0],
}
unsafe extern "C" {
    pub fn foo() -> ::std::os::raw::c_int;
}
