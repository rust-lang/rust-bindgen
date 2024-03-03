#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Wrapper_Wrapped<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub t: T,
}
impl<T> Default for Wrapper_Wrapped<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type Wrapper_Type<T> = Wrapper_Wrapped<T>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rooted<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub ptr: Rooted_MaybeWrapped<T>,
}
/// <div rustbindgen replaces="Rooted_MaybeWrapped"></div>
pub type Rooted_MaybeWrapped<T> = T;
impl<T> Default for Rooted<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
