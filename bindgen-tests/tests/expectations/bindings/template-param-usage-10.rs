#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DoublyIndirectUsage<T, U> {
    pub doubly_indirect: DoublyIndirectUsage_IndirectUsage<T, U>,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<U>>,
}
pub type DoublyIndirectUsage_Aliased<T> = T;
pub type DoublyIndirectUsage_Typedefed<U> = U;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DoublyIndirectUsage_IndirectUsage<T, U> {
    pub member: DoublyIndirectUsage_Aliased<T>,
    pub another: DoublyIndirectUsage_Typedefed<U>,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<U>>,
}
impl<T, U> Default for DoublyIndirectUsage_IndirectUsage<T, U> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl<T, U> Default for DoublyIndirectUsage<T, U> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
