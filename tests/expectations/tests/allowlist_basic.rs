#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AllowlistMe<T> {
    pub foo: ::std::os::raw::c_int,
    pub bar: AllowlistMe_Inner<T>,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AllowlistMe_Inner<T> {
    pub bar: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for AllowlistMe_Inner<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl<T> Default for AllowlistMe<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
