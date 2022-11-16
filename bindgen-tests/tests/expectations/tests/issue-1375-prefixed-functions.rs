#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern "C" {
    #[link_name = "\u{1}my_custom_prefix_var_const_name"]
    pub static var_const_name: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}my_custom_prefix_var_mut_name"]
    pub static mut var_mut_name: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}my_custom_prefix_function_name"]
    pub fn function_name(x: ::std::os::raw::c_int);
}
