#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo_InnerType<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub t: T,
}
impl<T> Default for Foo_InnerType<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type Bar = InnerType;
extern "C" {
    #[link_name = "\u{1}_Z4funcv"]
    pub fn func() -> Bar;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct InnerType {
    pub _address: u8,
}
