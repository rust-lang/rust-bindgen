#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern "C" {
    pub fn test_fn(
        a: f32,
        arr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn test_fn2(
        arr: *const f32,
        b: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub type defArr = [::std::os::raw::c_char; 20usize];
pub type foo =
    ::std::option::Option<unsafe extern "C" fn(a: *mut ::std::os::raw::c_char)>;
extern "C" {
    pub fn bar(a: *mut ::std::os::raw::c_char);
}
