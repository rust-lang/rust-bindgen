#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub const N0: u8 = 0;
pub const N1: u8 = 1;
pub const N2: u8 = 2;
pub const N_1: i8 = -1;
pub const N_2: i8 = -2;
pub const MAX_U16: u16 = 65535;
pub const MAX_I16: u16 = 32767;
pub const MAX_I16_Plus1: u16 = 32768;
pub const MAX_U16_Plus1: u32 = 65536;
pub const MAX_I16_Minus1: u16 = 32766;
pub const MAX_U16_Minus1: u16 = 65534;
pub const MIN_U16: u8 = 0;
pub const MIN_I16: i16 = -32768;
pub const MIN_U16_Plus1: u8 = 1;
pub const MIN_I16_Plus1: i16 = -32767;
pub const MIN_U16_Minus1: i8 = -1;
pub const MIN_I16_Minus1: i32 = -32769;
pub const MAX_U32: u32 = 4294967295;
pub const MAX_I32: u32 = 2147483647;
pub const MAX_I32_Plus1: u32 = 2147483648;
pub const MAX_U32_Plus1: u64 = 4294967296;
pub const MAX_I32_Minus1: u32 = 2147483646;
pub const MAX_U32_Minus1: u32 = 4294967294;
pub const MIN_U32: u8 = 0;
pub const MIN_I32: i32 = -2147483648;
pub const MIN_U32_Plus1: u8 = 1;
pub const MIN_I32_Plus1: i32 = -2147483647;
pub const MIN_U32_Minus1: i8 = -1;
pub const MIN_I32_Minus1: i64 = -2147483649;
pub const LONG12: u64 = 123456789012;
pub const LONG_12: i64 = -123456789012;
extern "C" {
    pub fn foo(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_uint,
        arg4: ::std::os::raw::c_char,
        arg5: ::std::os::raw::c_uchar,
        arg6: ::std::os::raw::c_schar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bar(
        arg1: ::std::os::raw::c_long,
        arg2: ::std::os::raw::c_longlong,
    ) -> ::std::os::raw::c_long;
}
