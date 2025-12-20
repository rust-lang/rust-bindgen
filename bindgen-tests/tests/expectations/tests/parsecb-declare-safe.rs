#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C" {
    pub safe static my_safe_var: [::std::os::raw::c_int; 3usize];
}
unsafe extern "C" {
    pub safe fn my_safe_func() -> ::std::os::raw::c_int;
}
