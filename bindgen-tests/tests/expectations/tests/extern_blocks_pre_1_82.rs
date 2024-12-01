#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
extern "C" {
    pub fn cool_function(i: ::std::os::raw::c_int, c: ::std::os::raw::c_char);
}
extern "C" {
    pub static mut cool_static: ::std::os::raw::c_int;
}
