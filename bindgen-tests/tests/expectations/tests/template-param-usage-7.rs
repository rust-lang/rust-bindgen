#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DoesNotUseU<T, V> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<V>>,
    pub t: T,
    pub v: V,
}
impl<T, V> Default for DoesNotUseU<T, V> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type Alias = DoesNotUseU<::std::os::raw::c_int, ::std::os::raw::c_char>;
