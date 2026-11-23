#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum small_value_t {
    SMALL_VALUE = 1,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of small_value_t"][::std::mem::size_of::<small_value_t>() - 4usize];
    ["Alignment of small_value_t"][::std::mem::align_of::<small_value_t>() - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum medium_value_t {
    MEDIUM_VALUE = 256,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of medium_value_t"][::std::mem::size_of::<medium_value_t>() - 4usize];
    ["Alignment of medium_value_t"][::std::mem::align_of::<medium_value_t>() - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum large_value_t {
    LARGE_VALUE = 16777216,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of large_value_t"][::std::mem::size_of::<large_value_t>() - 4usize];
    ["Alignment of large_value_t"][::std::mem::align_of::<large_value_t>() - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum unsigned_value_t {
    UNSIGNED_VALUE = 2147483648,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of unsigned_value_t"][::std::mem::size_of::<unsigned_value_t>() - 4usize];
    [
        "Alignment of unsigned_value_t",
    ][::std::mem::align_of::<unsigned_value_t>() - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum mixed_sign_t {
    NEGATIVE_VALUE = -1,
    POSITIVE_VALUE = 1,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of mixed_sign_t"][::std::mem::size_of::<mixed_sign_t>() - 4usize];
    ["Alignment of mixed_sign_t"][::std::mem::align_of::<mixed_sign_t>() - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum wide_mixed_sign_t {
    WIDE_NEGATIVE_VALUE = -1,
    WIDE_UNSIGNED_VALUE = 2147483648,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of wide_mixed_sign_t"][::std::mem::size_of::<wide_mixed_sign_t>() - 8usize];
    [
        "Alignment of wide_mixed_sign_t",
    ][::std::mem::align_of::<wide_mixed_sign_t>() - 8usize];
};
