#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type foo = ::std::option::Option<
    unsafe extern "C" fn(bar: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
>;
