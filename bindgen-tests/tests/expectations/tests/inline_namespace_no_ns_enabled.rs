#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug)]
pub struct std_basic_string<CharT> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<CharT>>,
    pub hider: std_basic_string_Alloc_hider,
    pub length: ::std::os::raw::c_ulong,
    pub __bindgen_anon_1: std_basic_string__bindgen_ty_1<CharT>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_basic_string_Alloc_hider {
    pub storage: *mut ::std::os::raw::c_void,
}
impl Default for std_basic_string_Alloc_hider {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct std_basic_string__bindgen_ty_1<CharT> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<CharT>>,
    pub inline_storage: [CharT; 4usize],
}
impl<CharT> Default for std_basic_string__bindgen_ty_1<CharT> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl<CharT> Default for std_basic_string<CharT> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
