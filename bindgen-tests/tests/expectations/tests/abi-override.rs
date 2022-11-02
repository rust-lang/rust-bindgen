#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern "fastcall" {
    pub fn foo();
}
extern "stdcall" {
    pub fn bar();
}
extern "C" {
    pub fn baz();
}
