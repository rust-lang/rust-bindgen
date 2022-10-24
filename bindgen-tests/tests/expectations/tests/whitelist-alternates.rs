#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct WhitelistedType {}
#[test]
fn bindgen_test_layout_WhitelistedType() {
    assert_eq!(
        ::std::mem::size_of::<WhitelistedType>(),
        0usize,
        concat!("Size of: ", stringify!(WhitelistedType))
    );
    assert_eq!(
        ::std::mem::align_of::<WhitelistedType>(),
        1usize,
        concat!("Alignment of ", stringify!(WhitelistedType))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AllowType {}
#[test]
fn bindgen_test_layout_AllowType() {
    assert_eq!(
        ::std::mem::size_of::<AllowType>(),
        0usize,
        concat!("Size of: ", stringify!(AllowType))
    );
    assert_eq!(
        ::std::mem::align_of::<AllowType>(),
        1usize,
        concat!("Alignment of ", stringify!(AllowType))
    );
}
