#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C" {
    pub fn foo() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn bar() -> !;
}
unsafe extern "C" {
    pub fn baz() -> !;
}
