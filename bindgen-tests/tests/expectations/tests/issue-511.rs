#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern "C" {
    pub static mut a: *mut ::std::os::raw::c_char;
}
extern "C" {
    pub static mut b: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static c: *mut ::std::os::raw::c_char;
}
extern "C" {
    pub static d: *const ::std::os::raw::c_char;
}
