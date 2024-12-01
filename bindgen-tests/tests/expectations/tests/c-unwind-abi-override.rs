#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C-unwind" {
    pub fn foo();
}
unsafe extern "C-unwind" {
    pub fn bar();
}
unsafe extern "C" {
    pub fn baz();
}
