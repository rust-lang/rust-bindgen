#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![cfg(not(test))]

pub type void_fn = ::std::option::Option<unsafe extern "stdcall" fn()>;
pub type fn_ = ::std::option::Option<
    unsafe extern "stdcall" fn(id: ::std::os::raw::c_int) -> void_fn,
>;
