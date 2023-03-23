#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Point {
    pub(crate) x: ::std::os::raw::c_int,
    pub(crate) y: ::std::os::raw::c_int,
}
