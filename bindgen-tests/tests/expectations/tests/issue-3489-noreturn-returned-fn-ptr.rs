#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type noreturn_fn_ptr = ::std::option::Option<
    unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> !,
>;
unsafe extern "C" {
    pub fn take(
        t: ::std::os::raw::c_int,
    ) -> ::std::option::Option<unsafe extern "C" fn(t: ::std::os::raw::c_int) -> !>;
}
unsafe extern "C" {
    pub fn take_noreturn(t: ::std::os::raw::c_int) -> !;
}
unsafe extern "C" {
    pub fn take_alias(t: ::std::os::raw::c_int) -> noreturn_fn_ptr;
}
unsafe extern "C" {
    pub fn take_alias_noreturn(t: ::std::os::raw::c_int) -> !;
}
unsafe extern "C" {
    pub fn plain(t: ::std::os::raw::c_int) -> !;
}
