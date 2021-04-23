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
extern "C" {
    #[link_name = "\u{1}_Z1fv"]
    pub fn f();
}
#[test]
fn __bindgen_test_layout_Foo_open0_Bar_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<Foo>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(Foo))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo>(),
        1usize,
        concat!("Alignment of template specialization: ", stringify!(Foo))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Baz {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Baz() {
    assert_eq!(
        ::std::mem::size_of::<Baz>(),
        1usize,
        concat!("Size of: ", stringify!(Baz))
    );
    assert_eq!(
        ::std::mem::align_of::<Baz>(),
        1usize,
        concat!("Alignment of ", stringify!(Baz))
    );
}
#[test]
fn __bindgen_test_layout_Foo_open0_Boo_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<Foo>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(Foo))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo>(),
        1usize,
        concat!("Alignment of template specialization: ", stringify!(Foo))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Bar {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(
        ::std::mem::size_of::<Bar>(),
        1usize,
        concat!("Size of: ", stringify!(Bar))
    );
    assert_eq!(
        ::std::mem::align_of::<Bar>(),
        1usize,
        concat!("Alignment of ", stringify!(Bar))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Boo {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Boo() {
    assert_eq!(
        ::std::mem::size_of::<Boo>(),
        1usize,
        concat!("Size of: ", stringify!(Boo))
    );
    assert_eq!(
        ::std::mem::align_of::<Boo>(),
        1usize,
        concat!("Alignment of ", stringify!(Boo))
    );
}
