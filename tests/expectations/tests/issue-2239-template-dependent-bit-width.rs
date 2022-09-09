#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct b {
    pub _address: u8,
}
pub type b_td<a> = a;
pub type b_ta<a> = a;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct b_foo {
    pub _address: u8,
}
