#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Point {
    pub x: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Point() {
    const UNINIT: ::std::mem::MaybeUninit<Point> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Point>(),
        4usize,
        concat!("Size of: ", stringify!(Point))
    );
    assert_eq!(
        ::std::mem::align_of::<Point>(),
        4usize,
        concat!("Alignment of ", stringify!(Point))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Point), "::", stringify!(x))
    );
}
extern "C" {
    pub fn foo() -> ::std::os::raw::c_int;
    pub fn bar() -> ::std::os::raw::c_int;
}
