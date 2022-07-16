#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NoDefault {
    pub i: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_NoDefault() {
    const UNINIT: ::std::mem::MaybeUninit<NoDefault> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<NoDefault>(),
        4usize,
        concat!("Size of: ", stringify!(NoDefault))
    );
    assert_eq!(
        ::std::mem::align_of::<NoDefault>(),
        4usize,
        concat!("Alignment of ", stringify!(NoDefault))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).i) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(NoDefault),
            "::",
            stringify!(i)
        )
    );
}
