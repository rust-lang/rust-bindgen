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
    pub fn foo() -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn bar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn baz() -> i32_;
}
extern "C" {
    pub fn qux() -> int32_;
}
