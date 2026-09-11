#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct base_t {
    pub __bindgen_anon_1: base_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct base_t__bindgen_ty_1 {
    pub aaa: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct base_t__bindgen_ty_1__bindgen_ty_1 {
    pub aaa: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of base_t__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::size_of::<base_t__bindgen_ty_1__bindgen_ty_1>() - 4usize];
    [
        "Alignment of base_t__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::align_of::<base_t__bindgen_ty_1__bindgen_ty_1>() - 4usize];
    [
        "Offset of field: base_t__bindgen_ty_1__bindgen_ty_1::aaa",
    ][::std::mem::offset_of!(base_t__bindgen_ty_1__bindgen_ty_1, aaa) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of base_t__bindgen_ty_1",
    ][::std::mem::size_of::<base_t__bindgen_ty_1>() - 4usize];
    [
        "Alignment of base_t__bindgen_ty_1",
    ][::std::mem::align_of::<base_t__bindgen_ty_1>() - 4usize];
    [
        "Offset of field: base_t__bindgen_ty_1::aaa",
    ][::std::mem::offset_of!(base_t__bindgen_ty_1, aaa) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of base_t"][::std::mem::size_of::<base_t>() - 4usize];
    ["Alignment of base_t"][::std::mem::align_of::<base_t>() - 4usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct mixed_t {
    pub __bindgen_anon_1: mixed_t__bindgen_ty_1,
    pub __bindgen_anon_2: mixed_t__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct mixed_t__bindgen_ty_1 {
    pub real_field: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct mixed_t__bindgen_ty_1__bindgen_ty_1 {
    pub phantom: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of mixed_t__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::size_of::<mixed_t__bindgen_ty_1__bindgen_ty_1>() - 4usize];
    [
        "Alignment of mixed_t__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::align_of::<mixed_t__bindgen_ty_1__bindgen_ty_1>() - 4usize];
    [
        "Offset of field: mixed_t__bindgen_ty_1__bindgen_ty_1::phantom",
    ][::std::mem::offset_of!(mixed_t__bindgen_ty_1__bindgen_ty_1, phantom) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of mixed_t__bindgen_ty_1",
    ][::std::mem::size_of::<mixed_t__bindgen_ty_1>() - 4usize];
    [
        "Alignment of mixed_t__bindgen_ty_1",
    ][::std::mem::align_of::<mixed_t__bindgen_ty_1>() - 4usize];
    [
        "Offset of field: mixed_t__bindgen_ty_1::real_field",
    ][::std::mem::offset_of!(mixed_t__bindgen_ty_1, real_field) - 0usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct mixed_t__bindgen_ty_2 {
    pub another_real: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of mixed_t__bindgen_ty_2",
    ][::std::mem::size_of::<mixed_t__bindgen_ty_2>() - 4usize];
    [
        "Alignment of mixed_t__bindgen_ty_2",
    ][::std::mem::align_of::<mixed_t__bindgen_ty_2>() - 4usize];
    [
        "Offset of field: mixed_t__bindgen_ty_2::another_real",
    ][::std::mem::offset_of!(mixed_t__bindgen_ty_2, another_real) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of mixed_t"][::std::mem::size_of::<mixed_t>() - 8usize];
    ["Alignment of mixed_t"][::std::mem::align_of::<mixed_t>() - 4usize];
};
