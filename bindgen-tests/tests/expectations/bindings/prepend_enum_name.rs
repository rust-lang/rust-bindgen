#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub const FOO_BAR: foo = 0;
pub const FOO_BAZ: foo = 1;
pub type foo = ::std::os::raw::c_uint;
