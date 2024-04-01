#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct false_type {
    pub _address: u8,
}
const _: () = {
    ["Size of false_type"][::std::mem::size_of::<false_type>() - 1usize];
    ["Alignment of false_type"][::std::mem::align_of::<false_type>() - 1usize];
};
