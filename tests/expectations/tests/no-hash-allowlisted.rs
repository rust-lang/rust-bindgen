#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct NoHash {
    pub i: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_NoHash() {
    assert_eq!(
        ::std::mem::size_of::<NoHash>(),
        4usize,
        concat!("Size of: ", stringify!(NoHash))
    );
    assert_eq!(
        ::std::mem::align_of::<NoHash>(),
        4usize,
        concat!("Alignment of ", stringify!(NoHash))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<NoHash>())).i as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(NoHash), "::", stringify!(i))
    );
}
