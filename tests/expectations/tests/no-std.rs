#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![no_std]
mod libc {
    pub type c_int = i32;
    pub enum c_void {}
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct foo {
    pub a: libc::c_int,
    pub b: libc::c_int,
    pub bar: *mut libc::c_void,
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::core::mem::size_of::<foo>(),
        16usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::core::mem::align_of::<foo>(),
        8usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<foo>())).a as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(a))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<foo>())).b as *const _ as usize },
        4usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(b))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<foo>())).bar as *const _ as usize },
        8usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(bar))
    );
}
impl Default for foo {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
