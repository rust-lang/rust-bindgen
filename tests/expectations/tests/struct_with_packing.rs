#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct a {
    pub b: ::std::os::raw::c_char,
    pub c: ::std::os::raw::c_short,
}
#[test]
fn bindgen_test_layout_a() {
    const UNINIT: ::std::mem::MaybeUninit<a> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<a>(),
        3usize,
        concat!("Size of: ", stringify!(a))
    );
    assert_eq!(
        ::std::mem::align_of::<a>(),
        1usize,
        concat!("Alignment of ", stringify!(a))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(a), "::", stringify!(b))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).c) as usize - ptr as usize },
        1usize,
        concat!("Offset of field: ", stringify!(a), "::", stringify!(c))
    );
}
