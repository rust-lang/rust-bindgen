#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![cfg(feature = "nightly")]
#![feature(abi_thiscall)]

extern "C-unwind" {
    pub fn foo();
}
extern "C-unwind" {
    pub fn bar();
}
extern "C" {
    pub fn baz();
}
