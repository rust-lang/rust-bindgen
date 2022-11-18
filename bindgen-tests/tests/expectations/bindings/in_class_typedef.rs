#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub _address: u8,
}
pub type Foo_elem_type<T> = T;
pub type Foo_ptr_type<T> = *mut T;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo_Bar {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
