#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C" {
    #[link_name = "\u{1}_ZL3foov"]
    pub fn foo() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZL3barv"]
    pub fn bar() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZL9takes_ptrPi"]
    pub fn takes_ptr(arg: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZL12takes_fn_ptrPFiiE"]
    pub fn takes_fn_ptr(
        f: ::std::option::Option<
            unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZL8takes_fnPFiiE"]
    pub fn takes_fn(
        f: ::std::option::Option<
            unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
pub type func = ::std::option::Option<
    unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
>;
unsafe extern "C" {
    #[link_name = "\u{1}_ZL11takes_aliasPFiiE"]
    pub fn takes_alias(f: func) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZL15takes_qualifiedPKPKi"]
    pub fn takes_qualified(
        arg: *const *const ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub const foo_BAR: foo = 0;
pub type foo = ::std::os::raw::c_uint;
unsafe extern "C" {
    #[link_name = "\u{1}_ZL10takes_enum3foo"]
    pub fn takes_enum(f: foo) -> foo;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZL9nevermorev"]
    pub fn nevermore();
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZL21takes_fn_with_no_argsPFivE"]
    pub fn takes_fn_with_no_args(
        f: ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZL17no_extra_argumentP13__va_list_tag"]
    pub fn no_extra_argument(va: *mut __va_list_tag);
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZL12many_va_listiP13__va_list_tagS0_"]
    pub fn many_va_list(
        i: ::std::os::raw::c_int,
        va1: *mut __va_list_tag,
        va2: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZL20wrap_as_variadic_fn1iP13__va_list_tag"]
    pub fn wrap_as_variadic_fn1_wrapped(
        i: ::std::os::raw::c_int,
        ...
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZL20wrap_as_variadic_fn2iP13__va_list_tag"]
    pub fn wrap_as_variadic_fn2_wrapped(i: ::std::os::raw::c_int, ...);
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN3quxL3fooEv"]
    pub fn qux_foo() -> ::std::os::raw::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
    _unused: [u8; 0],
}
