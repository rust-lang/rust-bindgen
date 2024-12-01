#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C" {
    #[link_name = "\u{1}foo_foo"]
    pub fn baz_foo() -> ::std::os::raw::c_int;
}
