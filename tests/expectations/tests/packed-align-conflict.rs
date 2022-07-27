#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct B__packed {
    pub a: u8,
    pub b: u16,
    pub c: u8,
}
#[derive(Debug, Default, Copy, Clone)]
#[repr(C, align(2))]
pub struct B(pub B__packed);
#[test]
fn bindgen_test_layout_B() {
    const UNINIT: ::std::mem::MaybeUninit<B> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr() as *const B__packed;
    assert_eq!(
        ::std::mem::size_of::<B>(),
        4usize,
        concat!("Size of: ", stringify!(B))
    );
    assert_eq!(
        ::std::mem::align_of::<B>(),
        2usize,
        concat!("Alignment of ", stringify!(B))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(B), "::", stringify!(a))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        1usize,
        concat!("Offset of field: ", stringify!(B), "::", stringify!(b))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).c) as usize - ptr as usize },
        3usize,
        concat!("Offset of field: ", stringify!(B), "::", stringify!(c))
    );
}
