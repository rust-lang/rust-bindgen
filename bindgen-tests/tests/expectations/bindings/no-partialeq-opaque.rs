#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct NoPartialEq {
    pub _bindgen_opaque_blob: u32,
}
#[test]
fn bindgen_test_layout_NoPartialEq() {
    assert_eq!(
        ::std::mem::size_of::<NoPartialEq>(),
        4usize,
        concat!("Size of: ", stringify!(NoPartialEq))
    );
    assert_eq!(
        ::std::mem::align_of::<NoPartialEq>(),
        4usize,
        concat!("Alignment of ", stringify!(NoPartialEq))
    );
}
