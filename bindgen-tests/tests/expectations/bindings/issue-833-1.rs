#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
pub struct nsTArray {
    pub hdr: *const (),
}

extern "C" {
    pub fn func() -> *mut nsTArray;
}
