#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C" {
    #[link_name = "\u{1}prefix_kept_in_link_name_only_var_coolConstVariable_name"]
    pub static var_coolConstVariable_name: ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}prefix_kept_in_link_name_only_var_coolVariable_name"]
    pub static mut var_coolVariable_name: ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z56prefix_kept_in_link_name_only_function_coolFunction_namei"]
    pub fn function_coolFunction_name(x: ::std::os::raw::c_int);
}
