#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![cfg(target_os = "macos")]
extern crate block;
pub type F = ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
pub type Blk = _bindgen_ty_id_26;
unsafe extern "C" {
    pub fn take(b: Blk, t: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
pub type DirectBlk = _bindgen_ty_id_28;
unsafe extern "C" {
    pub fn direct(b: DirectBlk, t: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
pub type _bindgen_ty_id_26 = *const ::block::Block<(::std::os::raw::c_int,), ()>;
pub type _bindgen_ty_id_28 = *const ::block::Block<(::std::os::raw::c_int,), ()>;
