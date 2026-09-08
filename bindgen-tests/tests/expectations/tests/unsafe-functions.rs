#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C" {
    pub safe fn is_zero(n: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn is_zero_ptr(n: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub safe fn is_number_5(n: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub safe fn is_number_42(n: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn is_number_other_than_8(n: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
