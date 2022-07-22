#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Foo {
    pub _address: u8,
}
pub type Foo_FunctionPtr<T> =
    ::std::option::Option<unsafe extern "C" fn() -> T>;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct RefPtr {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct RefPtr_Proxy {
    pub _address: u8,
}
pub type RefPtr_Proxy_member_function<R, Args> =
    ::std::option::Option<unsafe extern "C" fn(arg1: Args) -> R>;
pub type Returner<T> = ::std::option::Option<unsafe extern "C" fn() -> T>;
