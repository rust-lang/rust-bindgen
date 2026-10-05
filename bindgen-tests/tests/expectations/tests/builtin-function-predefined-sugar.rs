#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C" {
    pub fn strlen(s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_ulong;
}
