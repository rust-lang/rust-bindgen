#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const foo: &::std::ffi::CStr = {
    const BYTES: &[u8; 4] = b"bar\0";
    #[allow(unsafe_code)]
    unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(BYTES) }
};
pub const CHAR: ::std::os::raw::c_char = 'b' as ::std::os::raw::c_char;
pub const CHARR: ::std::os::raw::c_char = '\0' as ::std::os::raw::c_char;
pub const FLOAT: f32 = 5.09;
pub const FLOAT_EXPR: f32 = 0.005;
pub const LONG: u32 = 3;
pub const INVALID_UTF8: &::std::ffi::CStr = {
    const BYTES: &[u8; 5] = b"\xF0(\x8C(\0";
    #[allow(unsafe_code)]
    unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(BYTES) }
};
