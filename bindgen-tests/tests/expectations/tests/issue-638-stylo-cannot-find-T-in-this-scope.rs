#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RefPtr<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub use_of_t: T,
}
impl<T> Default for RefPtr<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UsesRefPtrWithAliasedTypeParam<U> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<U>>,
    pub member: RefPtr<UsesRefPtrWithAliasedTypeParam_V<U>>,
}
pub type UsesRefPtrWithAliasedTypeParam_V<U> = U;
impl<U> Default for UsesRefPtrWithAliasedTypeParam<U> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
