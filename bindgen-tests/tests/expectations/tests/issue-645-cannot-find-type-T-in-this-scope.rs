#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(Clone, Copy, Debug)]
pub struct RefPtr<T>(T);
#[repr(C)]
pub struct HasRefPtr<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub refptr_member: RefPtr<HasRefPtr_TypedefOfT<T>>,
}
pub type HasRefPtr_TypedefOfT<T> = T;
impl<T> Default for HasRefPtr<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
