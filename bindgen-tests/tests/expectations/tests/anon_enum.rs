#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Test {
    pub foo: ::std::os::raw::c_int,
    pub bar: f32,
}
pub const Test_T_NONE: Test__bindgen_ty_1 = Test__bindgen_ty_1::T_NONE;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Test__bindgen_ty_1 {
    T_NONE = 0,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Test"][::std::mem::size_of::<Test>() - 8usize];
    ["Alignment of Test"][::std::mem::align_of::<Test>() - 4usize];
    ["Offset of field: Test::foo"][::std::mem::offset_of!(Test, foo) - 0usize];
    ["Offset of field: Test::bar"][::std::mem::offset_of!(Test, bar) - 4usize];
};
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Baz {
    Foo = 0,
    Bar = 1,
}
