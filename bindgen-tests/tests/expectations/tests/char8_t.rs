#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C" {
    pub static mut global_array: [u8; 4usize];
}
unsafe extern "C" {
    pub static mut incomplete_array: [u8; 0usize];
}
unsafe extern "C" {
    pub static mut nested_array: [[u8; 2usize]; 2usize];
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct WithChar8Array {
    pub bytes: [u8; 8usize],
    pub single: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of WithChar8Array"][::std::mem::size_of::<WithChar8Array>() - 9usize];
    ["Alignment of WithChar8Array"][::std::mem::align_of::<WithChar8Array>() - 1usize];
    [
        "Offset of field: WithChar8Array::bytes",
    ][::std::mem::offset_of!(WithChar8Array, bytes) - 0usize];
    [
        "Offset of field: WithChar8Array::single",
    ][::std::mem::offset_of!(WithChar8Array, single) - 8usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_Z16take_char8_arrayPDu"]
    pub fn take_char8_array(input: *mut u8);
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z12return_char8Du"]
    pub fn return_char8(input: u8) -> u8;
}
pub const Char8Enum_High: Char8Enum = 255;
pub const Char8Enum_Low: Char8Enum = 65;
pub type Char8Enum = u8;
