#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type MaybeWrapped<A> = A;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rooted<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub ptr: MaybeWrapped<T>,
}
impl<T> Default for Rooted<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
