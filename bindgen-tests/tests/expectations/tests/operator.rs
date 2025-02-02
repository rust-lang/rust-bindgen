#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C" {
    #[link_name = "\u{1}_Z20operator_informationv"]
    pub fn operator_information() -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug)]
pub struct Foo {
    _unused: [u8; 0],
}
