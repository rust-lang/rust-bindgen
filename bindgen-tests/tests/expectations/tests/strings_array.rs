#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const MY_STRING_UTF8: &'static [u8; 14] = b"Hello, world!\0";
pub const MY_STRING_INTERIOR_NULL: &'static [u8; 7] = b"Hello,\0";
pub const MY_STRING_NON_UTF8: &'static [u8; 7] = b"ABCDE\xFF\0";
