#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern "C" {
    #[link_name = "\u{1}_Z11my_functioni"]
    pub fn my_function(a: ::std::os::raw::c_int);
}
extern "C" {
    #[link_name = "\u{1}_Z11my_functionPKc"]
    pub fn my_function1(a: *const ::std::os::raw::c_char);
}
