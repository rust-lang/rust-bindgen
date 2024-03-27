#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct BaseIgnoresT {
    pub x: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CrtpUsesU<U> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<U>>,
    pub _base: BaseIgnoresT,
    pub usage: U,
}
impl<U> Default for CrtpUsesU<U> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
