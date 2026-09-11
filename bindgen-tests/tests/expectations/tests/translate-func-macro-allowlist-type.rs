#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub x: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Foo"][::std::mem::size_of::<Foo>() - 4usize];
    ["Alignment of Foo"][::std::mem::align_of::<Foo>() - 4usize];
    ["Offset of field: Foo::x"][::std::mem::offset_of!(Foo, x) - 0usize];
};
#[allow(non_snake_case, unused_parens)]
pub const fn ADD(x: i64, y: i64) -> i64 {
    ((x) + (y))
}
