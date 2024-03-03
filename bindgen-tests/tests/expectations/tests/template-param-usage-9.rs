#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct DoesNotUse {
    pub _address: u8,
}
pub type DoesNotUse_Aliased<T> = T;
pub type DoesNotUse_Typedefed<U> = U;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DoesNotUse_IndirectUsage<T, U> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<U>>,
    pub member: DoesNotUse_Aliased<T>,
    pub another: DoesNotUse_Typedefed<U>,
}
impl<T, U> Default for DoesNotUse_IndirectUsage<T, U> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
