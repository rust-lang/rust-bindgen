#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct SizedIntegers {
    pub x: u8,
    pub y: u16,
    pub z: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of SizedIntegers"][::std::mem::size_of::<SizedIntegers>() - 8usize];
    ["Alignment of SizedIntegers"][::std::mem::align_of::<SizedIntegers>() - 4usize];
    [
        "Offset of field: SizedIntegers::x",
    ][::std::mem::offset_of!(SizedIntegers, x) - 0usize];
    [
        "Offset of field: SizedIntegers::y",
    ][::std::mem::offset_of!(SizedIntegers, y) - 2usize];
    [
        "Offset of field: SizedIntegers::z",
    ][::std::mem::offset_of!(SizedIntegers, z) - 4usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct StructWithBlocklistedFwdDecl {
    pub b: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of StructWithBlocklistedFwdDecl",
    ][::std::mem::size_of::<StructWithBlocklistedFwdDecl>() - 1usize];
    [
        "Alignment of StructWithBlocklistedFwdDecl",
    ][::std::mem::align_of::<StructWithBlocklistedFwdDecl>() - 1usize];
    [
        "Offset of field: StructWithBlocklistedFwdDecl::b",
    ][::std::mem::offset_of!(StructWithBlocklistedFwdDecl, b) - 0usize];
};
