#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(Clone, Copy, Debug)]
pub struct RefPtr<T>(T);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsMainThreadPtrHolder<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub a: T,
}
impl<T> Default for nsMainThreadPtrHolder<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct nsMainThreadPtrHandle<U> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<U>>,
    pub mPtr: RefPtr<nsMainThreadPtrHolder<U>>,
}
impl<U> Default for nsMainThreadPtrHandle<U> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
