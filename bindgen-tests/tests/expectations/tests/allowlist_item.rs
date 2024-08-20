#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const FooDefault: u32 = 0;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub field: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Foo"][::std::mem::size_of::<Foo>() - 4usize];
    ["Alignment of Foo"][::std::mem::align_of::<Foo>() - 4usize];
    ["Offset of field: Foo::field"][::std::mem::offset_of!(Foo, field) - 0usize];
};
extern "C" {
    pub fn FooNew(value: ::std::os::raw::c_int) -> Foo;
}
