#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern "C" {
    #[link_name = "\u{1}_Z3fooPKc"]
    pub fn foo(type_: *const ::std::os::raw::c_char);
}
