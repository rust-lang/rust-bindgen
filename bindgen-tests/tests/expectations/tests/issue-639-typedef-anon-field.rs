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
    ["Size of Foo_Bar"][::std::mem::size_of::<Foo_Bar>() - 4usize];
    ["Alignment of Foo_Bar"][::std::mem::align_of::<Foo_Bar>() - 4usize];
    ["Offset of field: Foo_Bar::abc"][::std::mem::offset_of!(Foo_Bar, abc) - 0usize];
};
const _: () = {
    ["Size of Foo"][::std::mem::size_of::<Foo>() - 4usize];
    ["Alignment of Foo"][::std::mem::align_of::<Foo>() - 4usize];
    ["Offset of field: Foo::bar"][::std::mem::offset_of!(Foo, bar) - 0usize];
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
    ["Size of Baz_Bar"][::std::mem::size_of::<Baz_Bar>() - 4usize];
    ["Alignment of Baz_Bar"][::std::mem::align_of::<Baz_Bar>() - 4usize];
    ["Offset of field: Baz_Bar::abc"][::std::mem::offset_of!(Baz_Bar, abc) - 0usize];
};
const _: () = {
    ["Size of Baz"][::std::mem::size_of::<Baz>() - 1usize];
    ["Alignment of Baz"][::std::mem::align_of::<Baz>() - 1usize];
};
