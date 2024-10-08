#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type one_byte_t_ctype = ::std::os::raw::c_uchar;
pub const one_byte_t_SOME_VALUE: one_byte_t_ctype = 1;
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum one_byte_t {
    SOME_VALUE = 1,
}
pub type two_byte_t_ctype = ::std::os::raw::c_ushort;
pub const two_byte_t_SOME_OTHER_VALUE: two_byte_t_ctype = 256;
#[repr(u16)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum two_byte_t {
    SOME_OTHER_VALUE = 256,
}
pub type four_byte_t_ctype = ::std::os::raw::c_uint;
pub const four_byte_t_SOME_BIGGER_VALUE: four_byte_t_ctype = 16777216;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum four_byte_t {
    SOME_BIGGER_VALUE = 16777216,
}
