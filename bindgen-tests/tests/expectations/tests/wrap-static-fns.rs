#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern "C" {
    #[link_name = "\u{1}foo__extern"]
    pub fn foo() -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}bar__extern"]
    pub fn bar() -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}takes_ptr__extern"]
    pub fn takes_ptr(arg: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}takes_fn_ptr__extern"]
    pub fn takes_fn_ptr(
        f: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}takes_fn__extern"]
    pub fn takes_fn(
        f: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
pub type func = ::std::option::Option<
    unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
>;
extern "C" {
    #[link_name = "\u{1}takes_alias__extern"]
    pub fn takes_alias(f: func) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}takes_qualified__extern"]
    pub fn takes_qualified(
        arg: *const *const ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
