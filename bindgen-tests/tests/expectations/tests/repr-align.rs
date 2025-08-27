#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![cfg(feature = "nightly")]
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct a {
    pub b: ::std::os::raw::c_int,
    pub c: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of a"][::std::mem::size_of::<a>() - 8usize];
    ["Alignment of a"][::std::mem::align_of::<a>() - 8usize];
    ["Offset of field: a::b"][::std::mem::offset_of!(a, b) - 0usize];
    ["Offset of field: a::c"][::std::mem::offset_of!(a, c) - 4usize];
};
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct b {
    pub b: ::std::os::raw::c_int,
    pub c: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of b"][::std::mem::size_of::<b>() - 8usize];
    ["Alignment of b"][::std::mem::align_of::<b>() - 8usize];
    ["Offset of field: b::b"][::std::mem::offset_of!(b, b) - 0usize];
    ["Offset of field: b::c"][::std::mem::offset_of!(b, c) - 4usize];
};
