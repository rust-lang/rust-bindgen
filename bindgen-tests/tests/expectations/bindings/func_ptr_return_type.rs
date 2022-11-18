#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern "C" {
    pub fn func() -> ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ::std::os::raw::c_int,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >;
}
