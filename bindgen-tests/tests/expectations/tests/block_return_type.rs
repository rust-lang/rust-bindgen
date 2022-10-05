#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![cfg(target_os = "macos")]

extern crate block;
extern "C" {
    pub fn func() -> _bindgen_ty_id_4;
}
pub type _bindgen_ty_id_4 = *const ::block::Block<
    (::std::os::raw::c_int, ::std::os::raw::c_int),
    ::std::os::raw::c_int,
>;
