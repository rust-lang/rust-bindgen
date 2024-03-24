#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct Foo {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(::std::mem::size_of::<Foo>(), 1usize, "Size of Foo");
    assert_eq!(::std::mem::align_of::<Foo>(), 1usize, "Alignment of Foo");
}
impl Clone for Foo {
    fn clone(&self) -> Self {
        *self
    }
}
