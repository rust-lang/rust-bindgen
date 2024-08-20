#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const ROOT: &[u8; 5] = b"root\0";
pub const A: ::std::os::raw::c_char = 97;
extern "C" {
    pub fn AA();
}
pub const B: u8 = 98u8;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct BB {}
const _: () = {
    ["Size of BB"][::std::mem::size_of::<BB>() - 0usize];
    ["Alignment of BB"][::std::mem::align_of::<BB>() - 1usize];
};
