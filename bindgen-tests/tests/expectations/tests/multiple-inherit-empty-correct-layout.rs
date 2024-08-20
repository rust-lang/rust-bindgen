#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Foo"][::std::mem::size_of::<Foo>() - 1usize];
    ["Alignment of Foo"][::std::mem::align_of::<Foo>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Bar {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Bar"][::std::mem::size_of::<Bar>() - 1usize];
    ["Alignment of Bar"][::std::mem::align_of::<Bar>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Baz {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Baz"][::std::mem::size_of::<Baz>() - 1usize];
    ["Alignment of Baz"][::std::mem::align_of::<Baz>() - 1usize];
};
