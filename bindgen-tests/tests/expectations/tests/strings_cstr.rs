#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(unsafe_code)]
pub const MY_STRING_UTF8: &::std::ffi::CStr = unsafe {
    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"Hello, world!\0")
};
#[allow(unsafe_code)]
pub const MY_STRING_INTERIOR_NULL: &::std::ffi::CStr = unsafe {
    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"Hello,\0")
};
#[allow(unsafe_code)]
pub const MY_STRING_NON_UTF8: &::std::ffi::CStr = unsafe {
    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"ABCDE\xFF\0")
};
