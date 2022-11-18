#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern "C" {
    pub fn foo();
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}?sBar@Foo@@2_NA"]
    pub static mut Foo_sBar: bool;
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(
        ::std::mem::size_of::<Foo>(),
        1usize,
        concat!("Size of: ", stringify!(Foo))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo>(),
        1usize,
        concat!("Alignment of ", stringify!(Foo))
    );
}
extern "fastcall" {
    pub fn fast_call_func_no_args() -> ::std::os::raw::c_int;
}
extern "fastcall" {
    pub fn fast_call_func_many_args(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "stdcall" {
    pub fn std_call_func_no_args() -> ::std::os::raw::c_int;
}
extern "stdcall" {
    pub fn std_call_func_many_args(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
