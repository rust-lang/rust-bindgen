#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
extern "C" {
    pub fn fun() -> u64;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Struct {
    pub field: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Struct"][::std::mem::size_of::<Struct>() - 8usize];
    ["Alignment of Struct"][::std::mem::align_of::<Struct>() - 8usize];
    ["Offset of field: Struct::field"][::std::mem::offset_of!(Struct, field) - 0usize];
};
