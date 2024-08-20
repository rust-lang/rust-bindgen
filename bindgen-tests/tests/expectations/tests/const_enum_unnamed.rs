#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const FOO_BAR: _bindgen_ty_1 = _bindgen_ty_1::FOO_BAR;
pub const FOO_BAZ: _bindgen_ty_1 = _bindgen_ty_1::FOO_BAZ;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_1 {
    FOO_BAR = 0,
    FOO_BAZ = 1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub _address: u8,
}
pub const Foo_FOO_BAR: Foo__bindgen_ty_1 = Foo__bindgen_ty_1::FOO_BAR;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Foo__bindgen_ty_1 {
    FOO_BAR = 10,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Foo"][::std::mem::size_of::<Foo>() - 1usize];
    ["Alignment of Foo"][::std::mem::align_of::<Foo>() - 1usize];
};
