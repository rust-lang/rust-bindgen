#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}_Z1fv"]
    pub fn f();
}
const _: () = {
    assert!(
        ::std::mem::size_of::<Foo>() == 1usize,
        "Size of template specialization: Foo_open0_Bar_close0",
    );
    assert!(
        ::std::mem::align_of::<Foo>() == 1usize,
        "Align of template specialization: Foo_open0_Bar_close0",
    );
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
const _: () = {
    assert!(
        ::std::mem::size_of::<Foo>() == 1usize,
        "Size of template specialization: Foo_open0_Boo_close0",
    );
    assert!(
        ::std::mem::align_of::<Foo>() == 1usize,
        "Align of template specialization: Foo_open0_Boo_close0",
    );
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
pub struct Boo {
    pub _address: u8,
}
const _: () = {
    assert!(::std::mem::size_of::<Boo>() == 1usize, "Size of Boo");
    assert!(::std::mem::align_of::<Boo>() == 1usize, "Alignment of Boo");
};
