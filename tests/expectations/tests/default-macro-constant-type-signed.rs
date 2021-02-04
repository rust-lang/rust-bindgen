#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub const N0: i32 = 0;
pub const N1: i32 = 1;
pub const N2: i32 = 2;
pub const N_1: i32 = -1;
pub const N_2: i32 = -2;
pub const MAX_U16: i32 = 65535;
pub const MAX_I16: i32 = 32767;
pub const MAX_I16_Plus1: i32 = 32768;
pub const MAX_U16_Plus1: i32 = 65536;
pub const MAX_I16_Minus1: i32 = 32766;
pub const MAX_U16_Minus1: i32 = 65534;
pub const MIN_U16: i32 = 0;
pub const MIN_I16: i32 = -32768;
pub const MIN_U16_Plus1: i32 = 1;
pub const MIN_I16_Plus1: i32 = -32767;
pub const MIN_U16_Minus1: i32 = -1;
pub const MIN_I16_Minus1: i32 = -32769;
pub const MAX_U32: i64 = 4294967295;
pub const MAX_I32: i32 = 2147483647;
pub const MAX_I32_Plus1: i64 = 2147483648;
pub const MAX_U32_Plus1: i64 = 4294967296;
pub const MAX_I32_Minus1: i32 = 2147483646;
pub const MAX_U32_Minus1: i64 = 4294967294;
pub const MIN_U32: i32 = 0;
pub const MIN_I32: i32 = -2147483648;
pub const MIN_U32_Plus1: i32 = 1;
pub const MIN_I32_Plus1: i32 = -2147483647;
pub const MIN_U32_Minus1: i32 = -1;
pub const MIN_I32_Minus1: i64 = -2147483649;
pub const LONG12: i64 = 123456789012;
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
