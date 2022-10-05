#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type i32_ = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct int32_ {
    pub inner: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_int32_() {
    const UNINIT: ::std::mem::MaybeUninit<int32_> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<int32_>(),
        4usize,
        concat!("Size of: ", stringify!(int32_))
    );
    assert_eq!(
        ::std::mem::align_of::<int32_>(),
        4usize,
        concat!("Alignment of ", stringify!(int32_))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inner) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(int32_),
            "::",
            stringify!(inner)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}_Z13returns_int64v"]
    pub fn returns_int64() -> ::std::os::raw::c_longlong;
}
extern "C" {
    #[link_name = "\u{1}_Z13returns_int32v"]
    pub fn returns_int32() -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z11returns_i32v"]
    pub fn returns_i32() -> i32_;
}
extern "C" {
    #[link_name = "\u{1}_Z14returns_int32_v"]
    pub fn returns_int32_() -> int32_;
}
extern "C" {
    #[link_name = "\u{1}_Z17returns_int32_ptrv"]
    pub fn returns_int32_ptr() -> *mut ::std::os::raw::c_int;
}
pub type integer<T> = T;
