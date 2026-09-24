#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// But the replacement type does use T!
///
/// <div rustbindgen replaces="JS::detail::MaybeWrapped" />
pub type JS_detail_MaybeWrapped<T> = T;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JS_Rooted<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub ptr: JS_detail_MaybeWrapped<T>,
}
impl<T> Default for JS_Rooted<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
