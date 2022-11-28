#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default)]
pub struct cv_Foo {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_cv_Foo() {
    assert_eq!(
        ::std::mem::size_of::<cv_Foo>(),
        1usize,
        concat!("Size of: ", stringify!(cv_Foo))
    );
    assert_eq!(
        ::std::mem::align_of::<cv_Foo>(),
        1usize,
        concat!("Alignment of ", stringify!(cv_Foo))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN2cv3FooD1Ev"]
    pub fn cv_Foo_Foo_destructor(this: *mut cv_Foo);
}
impl cv_Foo {
    #[inline]
    pub unsafe fn destruct(&mut self) {
        cv_Foo_Foo_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct cv_Bar {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_cv_Bar() {
    assert_eq!(
        ::std::mem::size_of::<cv_Bar>(),
        1usize,
        concat!("Size of: ", stringify!(cv_Bar))
    );
    assert_eq!(
        ::std::mem::align_of::<cv_Bar>(),
        1usize,
        concat!("Alignment of ", stringify!(cv_Bar))
    );
}
