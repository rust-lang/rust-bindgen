#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Proxy {
    pub _address: u8,
}
pub type Proxy_foo<T> =
    ::std::option::Option<unsafe extern "C" fn(bar: *mut T)>;
