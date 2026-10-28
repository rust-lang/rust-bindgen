#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C" {
    pub fn take(
        t: ::std::os::raw::c_int,
    ) -> ::std::option::Option<unsafe extern "C" fn(t: ::std::os::raw::c_int) -> !>;
}
unsafe extern "C" {
    pub fn plain(t: ::std::os::raw::c_int) -> !;
}
