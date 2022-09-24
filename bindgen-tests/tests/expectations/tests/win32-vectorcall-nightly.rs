#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![cfg(feature = "nightly")]
#![feature(abi_vectorcall)]

extern "vectorcall" {
    #[link_name = "\u{1}test_vectorcall@@16"]
    pub fn test_vectorcall(
        a: ::std::os::raw::c_int,
        b: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
