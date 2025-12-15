#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct __BindgenFloat16(pub u16);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct first_float16_struct {
    pub a: __BindgenFloat16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of first_float16_struct",
    ][::std::mem::size_of::<first_float16_struct>() - 2usize];
    [
        "Alignment of first_float16_struct",
    ][::std::mem::align_of::<first_float16_struct>() - 2usize];
    [
        "Offset of field: first_float16_struct::a",
    ][::std::mem::offset_of!(first_float16_struct, a) - 0usize];
};
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct second_float16_struct {
    pub b: __BindgenFloat16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of second_float16_struct",
    ][::std::mem::size_of::<second_float16_struct>() - 2usize];
    [
        "Alignment of second_float16_struct",
    ][::std::mem::align_of::<second_float16_struct>() - 2usize];
    [
        "Offset of field: second_float16_struct::b",
    ][::std::mem::offset_of!(second_float16_struct, b) - 0usize];
};
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct third_float16_struct {
    pub c: __BindgenFloat16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of third_float16_struct",
    ][::std::mem::size_of::<third_float16_struct>() - 2usize];
    [
        "Alignment of third_float16_struct",
    ][::std::mem::align_of::<third_float16_struct>() - 2usize];
    [
        "Offset of field: third_float16_struct::c",
    ][::std::mem::offset_of!(third_float16_struct, c) - 0usize];
};
