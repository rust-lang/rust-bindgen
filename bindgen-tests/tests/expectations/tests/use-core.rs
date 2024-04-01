#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![cfg(not(target_os = "windows"))]
extern crate core;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo {
    pub a: ::core::ffi::c_int,
    pub b: ::core::ffi::c_int,
    pub bar: *mut ::core::ffi::c_void,
}
const _: () = {
    ["Size of foo"][::core::mem::size_of::<foo>() - 16usize];
    ["Alignment of foo"][::core::mem::align_of::<foo>() - 8usize];
    ["Offset of field: foo::a"][::core::mem::offset_of!(foo, a) - 0usize];
    ["Offset of field: foo::b"][::core::mem::offset_of!(foo, b) - 4usize];
    ["Offset of field: foo::bar"][::core::mem::offset_of!(foo, bar) - 8usize];
};
impl Default for foo {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _bindgen_ty_1 {
    pub bar: ::core::ffi::c_int,
    pub baz: ::core::ffi::c_long,
}
const _: () = {
    ["Size of _bindgen_ty_1"][::core::mem::size_of::<_bindgen_ty_1>() - 8usize];
    ["Alignment of _bindgen_ty_1"][::core::mem::align_of::<_bindgen_ty_1>() - 8usize];
    [
        "Offset of field: _bindgen_ty_1::bar",
    ][::core::mem::offset_of!(_bindgen_ty_1, bar) - 0usize];
    [
        "Offset of field: _bindgen_ty_1::baz",
    ][::core::mem::offset_of!(_bindgen_ty_1, baz) - 0usize];
};
impl Default for _bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub static mut bazz: _bindgen_ty_1;
}
pub type fooFunction = ::core::option::Option<
    unsafe extern "C" fn(bar: ::core::ffi::c_int),
>;
