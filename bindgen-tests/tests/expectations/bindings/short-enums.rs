#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum one_byte_t {
    SOME_VALUE = 1,
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum two_byte_t {
    SOME_OTHER_VALUE = 256,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum four_byte_t {
    SOME_BIGGER_VALUE = 16777216,
}
