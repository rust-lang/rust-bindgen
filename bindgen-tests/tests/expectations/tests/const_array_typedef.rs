#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct strct {
    pub field: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_strct() {
    const UNINIT: ::std::mem::MaybeUninit<strct> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<strct>(),
        4usize,
        concat!("Size of: ", stringify!(strct)),
    );
    assert_eq!(
        ::std::mem::align_of::<strct>(),
        4usize,
        concat!("Alignment of ", stringify!(strct)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).field) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(strct), "::", stringify!(field)),
    );
}
pub type typ = [strct; 1usize];
extern "C" {
    pub static mut w: typ;
}
extern "C" {
    pub static mut x: *mut strct;
}
extern "C" {
    pub static y: typ;
}
extern "C" {
    pub static mut z: *const strct;
}
extern "C" {
    pub fn function(a: *const strct, b: *const strct);
}
