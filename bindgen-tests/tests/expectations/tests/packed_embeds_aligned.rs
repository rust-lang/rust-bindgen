#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[repr(align(2))]
#[derive(Debug, Default, Copy, Clone)]
pub struct inner1 {
    pub a: ::std::os::raw::c_char,
    pub b: ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_inner1() {
    const UNINIT: ::std::mem::MaybeUninit<inner1> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<inner1>(),
        2usize,
        concat!("Size of: ", stringify!(inner1)),
    );
    assert_eq!(
        ::std::mem::align_of::<inner1>(),
        2usize,
        concat!("Alignment of ", stringify!(inner1)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(inner1), "::", stringify!(a)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        1usize,
        concat!("Offset of field: ", stringify!(inner1), "::", stringify!(b)),
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct inner2 {
    pub a: inner1,
    pub b: inner1,
}
#[test]
fn bindgen_test_layout_inner2() {
    const UNINIT: ::std::mem::MaybeUninit<inner2> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<inner2>(),
        4usize,
        concat!("Size of: ", stringify!(inner2)),
    );
    assert_eq!(
        ::std::mem::align_of::<inner2>(),
        2usize,
        concat!("Alignment of ", stringify!(inner2)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(inner2), "::", stringify!(a)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        2usize,
        concat!("Offset of field: ", stringify!(inner2), "::", stringify!(b)),
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct outer1 {
    pub a: ::std::os::raw::c_short,
    pub b: inner1,
}
#[test]
fn bindgen_test_layout_outer1() {
    const UNINIT: ::std::mem::MaybeUninit<outer1> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<outer1>(),
        4usize,
        concat!("Size of: ", stringify!(outer1)),
    );
    assert_eq!(
        ::std::mem::align_of::<outer1>(),
        2usize,
        concat!("Alignment of ", stringify!(outer1)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(outer1), "::", stringify!(a)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        2usize,
        concat!("Offset of field: ", stringify!(outer1), "::", stringify!(b)),
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct outer2 {
    pub a: ::std::os::raw::c_short,
    pub b: inner2,
}
#[test]
fn bindgen_test_layout_outer2() {
    const UNINIT: ::std::mem::MaybeUninit<outer2> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<outer2>(),
        6usize,
        concat!("Size of: ", stringify!(outer2)),
    );
    assert_eq!(
        ::std::mem::align_of::<outer2>(),
        2usize,
        concat!("Alignment of ", stringify!(outer2)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(outer2), "::", stringify!(a)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        2usize,
        concat!("Offset of field: ", stringify!(outer2), "::", stringify!(b)),
    );
}
