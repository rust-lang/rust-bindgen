#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// This is intended to replace another type, but won't if we treat this include
/// as a system include, because clang doesn't parse comments there.
///
/// See #848.
///
/// <div rustbindgen replaces="nsTArray"></div>
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsTArray<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub m: *mut T,
}
impl<T> Default for nsTArray<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn func() -> *mut nsTArray<::std::os::raw::c_int>;
}
