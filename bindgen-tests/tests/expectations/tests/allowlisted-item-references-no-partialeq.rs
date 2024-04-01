#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct NoPartialEq {
    pub _address: u8,
}
const _: () = {
    ["Size of NoPartialEq"][::std::mem::size_of::<NoPartialEq>() - 1usize];
    ["Alignment of NoPartialEq"][::std::mem::align_of::<NoPartialEq>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AllowlistMe {
    pub a: NoPartialEq,
}
const _: () = {
    ["Size of AllowlistMe"][::std::mem::size_of::<AllowlistMe>() - 1usize];
    ["Alignment of AllowlistMe"][::std::mem::align_of::<AllowlistMe>() - 1usize];
    ["Offset of field: AllowlistMe::a"][::std::mem::offset_of!(AllowlistMe, a) - 0usize];
};
