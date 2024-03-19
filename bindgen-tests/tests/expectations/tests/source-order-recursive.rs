#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct foo {}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        0usize,
        concat!("Size of: ", stringify!(foo)),
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        1usize,
        concat!("Alignment of ", stringify!(foo)),
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bar {
    pub field: foo,
}
#[test]
fn bindgen_test_layout_bar() {
    const UNINIT: ::std::mem::MaybeUninit<bar> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<bar>(),
        0usize,
        concat!("Size of: ", stringify!(bar)),
    );
    assert_eq!(
        ::std::mem::align_of::<bar>(),
        1usize,
        concat!("Alignment of ", stringify!(bar)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).field) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(bar), "::", stringify!(field)),
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct baz {
    pub field: bar,
}
#[test]
fn bindgen_test_layout_baz() {
    const UNINIT: ::std::mem::MaybeUninit<baz> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<baz>(),
        0usize,
        concat!("Size of: ", stringify!(baz)),
    );
    assert_eq!(
        ::std::mem::align_of::<baz>(),
        1usize,
        concat!("Alignment of ", stringify!(baz)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).field) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(baz), "::", stringify!(field)),
    );
}
