#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern "C" {
    pub static mut foo: *mut [::std::os::raw::c_int; 1usize];
}
