#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct first_complex_struct {
    pub a: __BindgenComplex<f64>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of first_complex_struct",
    ][::std::mem::size_of::<first_complex_struct>() - 16usize];
    [
        "Alignment of first_complex_struct",
    ][::std::mem::align_of::<first_complex_struct>() - 8usize];
    [
        "Offset of field: first_complex_struct::a",
    ][::std::mem::offset_of!(first_complex_struct, a) - 0usize];
};
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct second_complex_struct {
    pub b: __BindgenComplex<f32>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of second_complex_struct",
    ][::std::mem::size_of::<second_complex_struct>() - 8usize];
    [
        "Alignment of second_complex_struct",
    ][::std::mem::align_of::<second_complex_struct>() - 4usize];
    [
        "Offset of field: second_complex_struct::b",
    ][::std::mem::offset_of!(second_complex_struct, b) - 0usize];
};
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct third_complex_struct {
    pub c: __BindgenComplex<f64>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of third_complex_struct",
    ][::std::mem::size_of::<third_complex_struct>() - 16usize];
    [
        "Alignment of third_complex_struct",
    ][::std::mem::align_of::<third_complex_struct>() - 8usize];
    [
        "Offset of field: third_complex_struct::c",
    ][::std::mem::offset_of!(third_complex_struct, c) - 0usize];
};
