#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct NoHash {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of NoHash"][::std::mem::size_of::<NoHash>() - 1usize];
    ["Alignment of NoHash"][::std::mem::align_of::<NoHash>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AllowlistMe {
    pub a: NoHash,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of AllowlistMe"][::std::mem::size_of::<AllowlistMe>() - 1usize];
    ["Alignment of AllowlistMe"][::std::mem::align_of::<AllowlistMe>() - 1usize];
    ["Offset of field: AllowlistMe::a"][::std::mem::offset_of!(AllowlistMe, a) - 0usize];
};
