#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndirectUsage<T, U> {
    pub member1: IndirectUsage_Typedefed<T>,
    pub member2: IndirectUsage_Aliased<U>,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<U>>,
}
pub type IndirectUsage_Typedefed<T> = T;
pub type IndirectUsage_Aliased<U> = U;
impl<T, U> Default for IndirectUsage<T, U> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
