#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default)]
pub struct cv_String {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_cv_String() {
    assert_eq!(
        ::std::mem::size_of::<cv_String>(),
        1usize,
        concat!("Size of: ", stringify!(cv_String))
    );
    assert_eq!(
        ::std::mem::align_of::<cv_String>(),
        1usize,
        concat!("Alignment of ", stringify!(cv_String))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2cv6StringD1Ev"]
    pub fn cv_String_String_destructor(this: *mut cv_String);
}
impl cv_String {
    #[inline]
    pub unsafe fn destruct(&mut self) {
        unsafe { cv_String_String_destructor(self) }
    }
}
