#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern "C" {
    #[link_name = "\u{1}foo__extern"]
    pub fn foo() -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}bar__extern"]
    pub fn bar() -> ::std::os::raw::c_int;
}
