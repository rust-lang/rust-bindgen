#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const THIS_SHOULD_BE_FIRST: ::std::os::raw::c_int = 1;
extern "C" {
    pub fn THIS_SHOULD_BE_SECOND();
}
pub type THIS_SHOULD_BE_THIRD = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct THIS_SHOULD_BE_FOURTH {}
#[test]
fn bindgen_test_layout_THIS_SHOULD_BE_FOURTH() {
    assert_eq!(
        ::std::mem::size_of::<THIS_SHOULD_BE_FOURTH>(),
        0usize,
        concat!("Size of: ", stringify!(THIS_SHOULD_BE_FOURTH)),
    );
    assert_eq!(
        ::std::mem::align_of::<THIS_SHOULD_BE_FOURTH>(),
        1usize,
        concat!("Alignment of ", stringify!(THIS_SHOULD_BE_FOURTH)),
    );
}
extern "C" {
    pub static mut THIS_SHOULD_BE_FIFTH: ::std::os::raw::c_int;
}
