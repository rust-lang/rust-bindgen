#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct false_type {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_false_type() {
    assert_eq!(
        ::std::mem::size_of::<false_type>(),
        1usize,
        concat!("Size of: ", stringify!(false_type))
    );
    assert_eq!(
        ::std::mem::align_of::<false_type>(),
        1usize,
        concat!("Alignment of ", stringify!(false_type))
    );
}
