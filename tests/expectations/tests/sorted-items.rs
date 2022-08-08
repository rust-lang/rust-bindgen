#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern "C" {
    pub fn foo() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bar(x: number) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn baz(point: Point) -> ::std::os::raw::c_int;
}
#[test]
fn bindgen_test_layout_Point() {
    assert_eq!(
        ::std::mem::size_of::<Point>(),
        8usize,
        concat!("Size of: ", stringify!(Point))
    );
    assert_eq!(
        ::std::mem::align_of::<Point>(),
        4usize,
        concat!("Alignment of ", stringify!(Point))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Point>())).x as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Point), "::", stringify!(x))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Point>())).y as *const _ as usize },
        4usize,
        concat!("Offset of field: ", stringify!(Point), "::", stringify!(y))
    );
}
pub const NUMBER: number = 42;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: number,
    pub y: number,
}
pub type number = ::std::os::raw::c_int;
