#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
pub struct declare_union {
    _unused: [u8; 0],
}
#[repr(C)]
pub union define_union<value_t> {
    pub value: *mut value_t,
    pub dummy: ::std::os::raw::c_int,
    pub _phantom_0:
        ::std::marker::PhantomData<::std::cell::UnsafeCell<value_t>>,
}
impl<value_t> Default for define_union<value_t> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
