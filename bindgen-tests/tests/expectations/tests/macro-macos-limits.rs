#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const CHAR_BIT: u32 = 8;
pub const MB_LEN_MAX: u32 = 1;
pub const SCHAR_MIN: i32 = -128;
pub const SCHAR_MAX: u32 = 127;
pub const UCHAR_MAX: u32 = 255;
pub const CHAR_MIN: i32 = -128;
pub const CHAR_MAX: u32 = 127;
pub const SHRT_MIN: i32 = -32768;
pub const SHRT_MAX: u32 = 32767;
pub const USHRT_MAX: u32 = 65535;
pub const INT_MIN: ::std::os::raw::c_int = -::std::os::raw::c_int::MAX - 1;
pub const INT_MAX: ::std::os::raw::c_int = ::std::os::raw::c_int::MAX;
pub const UINT_MAX: ::std::os::raw::c_uint = ::std::os::raw::c_int::MAX
    as ::std::os::raw::c_uint * 2 + 1;
pub const LONG_MIN: ::std::os::raw::c_long = -::std::os::raw::c_long::MAX - 1;
pub const LONG_MAX: ::std::os::raw::c_long = ::std::os::raw::c_long::MAX;
pub const ULONG_MAX: ::std::os::raw::c_ulong = ::std::os::raw::c_long::MAX
    as ::std::os::raw::c_ulong * 2 + 1;
pub const LONG_LONG_MIN: ::std::os::raw::c_longlong = -::std::os::raw::c_longlong::MAX
    - 1;
pub const LONG_LONG_MAX: ::std::os::raw::c_longlong = ::std::os::raw::c_longlong::MAX;
pub const ULONG_LONG_MAX: ::std::os::raw::c_ulonglong = ::std::os::raw::c_longlong::MAX
    as ::std::os::raw::c_ulonglong * 2 + 1;
