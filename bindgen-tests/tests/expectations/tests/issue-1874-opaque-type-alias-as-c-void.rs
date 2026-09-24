#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug)]
pub struct _WINDOW {
    _unused: [u8; 0],
}
pub type WINDOW = ::std::os::raw::c_void;
unsafe extern "C" {
    pub fn wgetch(win: *mut WINDOW) -> ::std::os::raw::c_int;
}
