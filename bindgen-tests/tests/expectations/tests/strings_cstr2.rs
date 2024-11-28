#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const MY_STRING_UTF8: &::std::ffi::CStr = c"Hello, world!";
pub const MY_STRING_INTERIOR_NULL: &::std::ffi::CStr = c"Hello,";
pub const MY_STRING_NON_UTF8: &::std::ffi::CStr = c"ABCDE\xFF";
