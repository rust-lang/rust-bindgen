#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Outer {
    pub i: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AutoIdVector {
    pub ar: Outer,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of AutoIdVector"][::std::mem::size_of::<AutoIdVector>() - 1usize];
    ["Alignment of AutoIdVector"][::std::mem::align_of::<AutoIdVector>() - 1usize];
    [
        "Offset of field: AutoIdVector::ar",
    ][::std::mem::offset_of!(AutoIdVector, ar) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Outer_open0_int_close0",
    ][::std::mem::size_of::<Outer>() - 1usize];
    [
        "Align of template specialization: Outer_open0_int_close0",
    ][::std::mem::align_of::<Outer>() - 1usize];
};
