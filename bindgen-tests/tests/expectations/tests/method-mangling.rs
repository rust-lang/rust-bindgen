#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(
        ::std::mem::size_of::<Foo>(),
        1usize,
        concat!("Size of: ", stringify!(Foo))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo>(),
        1usize,
        concat!("Alignment of ", stringify!(Foo))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN3Foo4typeEv"]
    pub fn Foo_type(this: *mut Foo) -> ::std::os::raw::c_int;
}
impl Foo {
    #[inline]
    pub unsafe fn type_(&mut self) -> ::std::os::raw::c_int {
        Foo_type(self)
    }
}
