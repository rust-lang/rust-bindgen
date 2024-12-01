#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C" {
    #[link_name = "\u{1}foo_bar"]
    pub fn bar() -> ::std::os::raw::c_int;
}
