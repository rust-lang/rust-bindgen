#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct a {
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_a() {
    const UNINIT: ::std::mem::MaybeUninit<a> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<a>(),
        4usize,
        concat!("Size of: ", stringify!(a))
    );
    assert_eq!(
        ::std::mem::align_of::<a>(),
        4usize,
        concat!("Alignment of ", stringify!(a))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(a), "::", stringify!(b))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct c {
    pub d: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_c() {
    const UNINIT: ::std::mem::MaybeUninit<c> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<c>(),
        4usize,
        concat!("Size of: ", stringify!(c))
    );
    assert_eq!(
        ::std::mem::align_of::<c>(),
        4usize,
        concat!("Alignment of ", stringify!(c))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).d) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(c), "::", stringify!(d))
    );
}
