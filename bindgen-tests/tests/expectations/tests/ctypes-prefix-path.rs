#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![no_std]
mod libc {
    pub mod foo {
        pub type c_int = i32;
        pub enum c_void {}
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct foo {
    pub a: libc::foo::c_int,
    pub b: libc::foo::c_int,
    pub bar: *mut libc::foo::c_void,
}
#[test]
fn bindgen_test_layout_foo() {
    const UNINIT: ::core::mem::MaybeUninit<foo> =
        ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
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
        unsafe { ::core::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(a))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        4usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(b))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).bar) as usize - ptr as usize },
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
