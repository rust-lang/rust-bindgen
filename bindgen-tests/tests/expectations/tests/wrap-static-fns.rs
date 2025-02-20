#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C" {
    #[link_name = "foo__extern"]
    pub fn foo() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "bar__extern"]
    pub fn bar() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "takes_ptr__extern"]
    pub fn takes_ptr(arg: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "takes_fn_ptr__extern"]
    pub fn takes_fn_ptr(
        f: ::std::option::Option<
            unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "takes_fn__extern"]
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
    #[link_name = "takes_alias__extern"]
    pub fn takes_alias(f: func) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "takes_qualified__extern"]
    pub fn takes_qualified(
        arg: *const *const ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub const foo_BAR: foo = 0;
pub type foo = ::std::os::raw::c_uint;
unsafe extern "C" {
    #[link_name = "takes_enum__extern"]
    pub fn takes_enum(f: foo) -> foo;
}
unsafe extern "C" {
    #[link_name = "nevermore__extern"]
    pub fn nevermore();
}
unsafe extern "C" {
    #[link_name = "takes_fn_with_no_args__extern"]
    pub fn takes_fn_with_no_args(
        f: ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "no_extra_argument__extern"]
    pub fn no_extra_argument(va: *mut __va_list_tag);
}
unsafe extern "C" {
    #[link_name = "many_va_list__extern"]
    pub fn many_va_list(
        i: ::std::os::raw::c_int,
        va1: *mut __va_list_tag,
        va2: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "wrap_as_variadic_fn1__extern"]
    pub fn wrap_as_variadic_fn1_wrapped(
        i: ::std::os::raw::c_int,
        ...
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "wrap_as_variadic_fn2__extern"]
    pub fn wrap_as_variadic_fn2_wrapped(i: ::std::os::raw::c_int, ...);
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
    _unused: [u8; 0],
}
