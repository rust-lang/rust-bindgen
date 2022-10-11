#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct foo {
    pub inner: ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_foo() {
    const UNINIT: ::std::mem::MaybeUninit<foo> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        1usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        1usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inner) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(foo),
            "::",
            stringify!(inner)
        )
    );
}
pub type foo_ptr = *const foo;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bar {
    pub inner: ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_bar() {
    const UNINIT: ::std::mem::MaybeUninit<bar> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<bar>(),
        1usize,
        concat!("Size of: ", stringify!(bar))
    );
    assert_eq!(
        ::std::mem::align_of::<bar>(),
        1usize,
        concat!("Alignment of ", stringify!(bar))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inner) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bar),
            "::",
            stringify!(inner)
        )
    );
}
pub type bar_ptr = *mut bar;
extern "C" {
    pub fn takes_foo_ptr(arg1: foo_ptr);
}
extern "C" {
    pub fn takes_foo_struct(arg1: foo);
}
extern "C" {
    pub fn takes_bar_ptr(arg1: bar_ptr);
}
extern "C" {
    pub fn takes_bar_struct(arg1: bar);
}
