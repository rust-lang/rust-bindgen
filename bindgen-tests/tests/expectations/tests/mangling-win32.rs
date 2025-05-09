#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![cfg(target = "i686-pc-windows-msvc")]
unsafe extern "C" {
    pub fn foo();
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub _address: u8,
}
unsafe extern "C" {
    #[link_name = "\u{1}?sBar@Foo@@2_NA"]
    pub static mut Foo_sBar: bool;
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Foo"][::std::mem::size_of::<Foo>() - 1usize];
    ["Alignment of Foo"][::std::mem::align_of::<Foo>() - 1usize];
};
unsafe extern "fastcall" {
    pub fn fast_call_func_no_args() -> ::std::os::raw::c_int;
}
unsafe extern "fastcall" {
    pub fn fast_call_func_many_args(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "stdcall" {
    pub fn std_call_func_no_args() -> ::std::os::raw::c_int;
}
unsafe extern "stdcall" {
    pub fn std_call_func_many_args(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
