#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern "C" {
    pub fn foo() -> ::std::os::raw::c_int;
}
pub type number = ::std::os::raw::c_int;
extern "C" {
    pub fn bar(x: number) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Point {
    pub x: number,
    pub y: number,
}
#[test]
fn bindgen_test_layout_Point() {
    const UNINIT: ::std::mem::MaybeUninit<Point> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
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
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Point), "::", stringify!(x))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        4usize,
        concat!("Offset of field: ", stringify!(Point), "::", stringify!(y))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Angle {
    pub a: number,
    pub b: number,
}
#[test]
fn bindgen_test_layout_Angle() {
    const UNINIT: ::std::mem::MaybeUninit<Angle> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Angle>(),
        8usize,
        concat!("Size of: ", stringify!(Angle))
    );
    assert_eq!(
        ::std::mem::align_of::<Angle>(),
        4usize,
        concat!("Alignment of ", stringify!(Angle))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Angle), "::", stringify!(a))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        4usize,
        concat!("Offset of field: ", stringify!(Angle), "::", stringify!(b))
    );
}
extern "C" {
    pub fn baz(point: Point) -> ::std::os::raw::c_int;
}
pub const NUMBER: number = 42;
