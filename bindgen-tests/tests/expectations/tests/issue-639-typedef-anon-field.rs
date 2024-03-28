#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub bar: Foo_Bar,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo_Bar {
    pub abc: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<Foo_Bar>() == 4usize, "Size of Foo_Bar");
    assert!(::std::mem::align_of::<Foo_Bar>() == 4usize, "Alignment of Foo_Bar");
    assert!(
        ::std::mem::offset_of!(Foo_Bar, abc) == 0usize,
        "Offset of field: Foo_Bar::abc",
    );
};
const _: () = {
    assert!(::std::mem::size_of::<Foo>() == 4usize, "Size of Foo");
    assert!(::std::mem::align_of::<Foo>() == 4usize, "Alignment of Foo");
    assert!(::std::mem::offset_of!(Foo, bar) == 0usize, "Offset of field: Foo::bar");
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Baz {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Baz_Bar {
    pub abc: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<Baz_Bar>() == 4usize, "Size of Baz_Bar");
    assert!(::std::mem::align_of::<Baz_Bar>() == 4usize, "Alignment of Baz_Bar");
    assert!(
        ::std::mem::offset_of!(Baz_Bar, abc) == 0usize,
        "Offset of field: Baz_Bar::abc",
    );
};
const _: () = {
    assert!(::std::mem::size_of::<Baz>() == 1usize, "Size of Baz");
    assert!(::std::mem::align_of::<Baz>() == 1usize, "Alignment of Baz");
};
