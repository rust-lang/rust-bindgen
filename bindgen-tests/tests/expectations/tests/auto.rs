#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub _address: u8,
}
pub const Foo_kFoo: bool = true;
const _: () = {
    assert!(::std::mem::size_of::<Foo>() == 1usize, "Size of Foo");
    assert!(::std::mem::align_of::<Foo>() == 1usize, "Alignment of Foo");
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Bar {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}_Z5Test2v"]
    pub fn Test2() -> ::std::os::raw::c_uint;
}
