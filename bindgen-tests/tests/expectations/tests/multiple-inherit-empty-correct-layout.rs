#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub _address: u8,
}
const _: () = {
    assert!(::std::mem::size_of::<Foo>() == 1usize, "Size of Foo");
    assert!(::std::mem::align_of::<Foo>() == 1usize, "Alignment of Foo");
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Bar {
    pub _address: u8,
}
const _: () = {
    assert!(::std::mem::size_of::<Bar>() == 1usize, "Size of Bar");
    assert!(::std::mem::align_of::<Bar>() == 1usize, "Alignment of Bar");
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Baz {
    pub _address: u8,
}
const _: () = {
    assert!(::std::mem::size_of::<Baz>() == 1usize, "Size of Baz");
    assert!(::std::mem::align_of::<Baz>() == 1usize, "Alignment of Baz");
};
