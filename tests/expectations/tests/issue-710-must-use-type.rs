#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct MyType {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OtherType {
    _unused: [u8; 0],
}
