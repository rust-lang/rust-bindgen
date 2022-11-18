#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
pub struct nsTArray<T> {
    pub hdr: *const T,
}

extern "C" {
    pub fn func() -> *mut nsTArray<::std::os::raw::c_int>;
}
