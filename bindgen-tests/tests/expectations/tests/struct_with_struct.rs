#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo {
    pub bar: foo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_1 {
    pub x: ::std::os::raw::c_uint,
    pub y: ::std::os::raw::c_uint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of foo__bindgen_ty_1"][::std::mem::size_of::<foo__bindgen_ty_1>() - 8usize];
    [
        "Alignment of foo__bindgen_ty_1",
    ][::std::mem::align_of::<foo__bindgen_ty_1>() - 4usize];
    [
        "Offset of field: foo__bindgen_ty_1::x",
    ][::std::mem::offset_of!(foo__bindgen_ty_1, x) - 0usize];
    [
        "Offset of field: foo__bindgen_ty_1::y",
    ][::std::mem::offset_of!(foo__bindgen_ty_1, y) - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of foo"][::std::mem::size_of::<foo>() - 8usize];
    ["Alignment of foo"][::std::mem::align_of::<foo>() - 4usize];
    ["Offset of field: foo::bar"][::std::mem::offset_of!(foo, bar) - 0usize];
};
