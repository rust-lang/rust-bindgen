#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(dead_code)]
unsafe extern "C" {
    pub fn test_function();
    pub static mut test_var: ::std::os::raw::c_int;
    pub static test_const_var: ::std::os::raw::c_int;
}
