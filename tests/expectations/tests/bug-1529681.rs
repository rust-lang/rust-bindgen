#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct BrowsingContext {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_BrowsingContext() {
    assert_eq!(
        ::std::mem::size_of::<BrowsingContext>(),
        1usize,
        concat!("Size of: ", stringify!(BrowsingContext))
    );
    assert_eq!(
        ::std::mem::align_of::<BrowsingContext>(),
        1usize,
        concat!("Alignment of ", stringify!(BrowsingContext))
    );
}
