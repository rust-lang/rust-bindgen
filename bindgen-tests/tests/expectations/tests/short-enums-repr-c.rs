#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum one_byte_t {
    SOME_VALUE = 1,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of one_byte_t"][::std::mem::size_of::<one_byte_t>() - 1usize];
    ["Alignment of one_byte_t"][::std::mem::align_of::<one_byte_t>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum two_byte_t {
    SOME_OTHER_VALUE = 256,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of two_byte_t"][::std::mem::size_of::<two_byte_t>() - 2usize];
    ["Alignment of two_byte_t"][::std::mem::align_of::<two_byte_t>() - 2usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum four_byte_t {
    SOME_BIGGER_VALUE = 16777216,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of four_byte_t"][::std::mem::size_of::<four_byte_t>() - 4usize];
    ["Alignment of four_byte_t"][::std::mem::align_of::<four_byte_t>() - 4usize];
};
