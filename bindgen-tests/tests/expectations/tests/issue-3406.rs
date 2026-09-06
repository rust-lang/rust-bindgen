#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
pub struct Inner {
    pub byte: ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Inner"][::std::mem::size_of::<Inner>() - 16usize];
    ["Alignment of Inner"][::std::mem::align_of::<Inner>() - 16usize];
    ["Offset of field: Inner::byte"][::std::mem::offset_of!(Inner, byte) - 0usize];
};
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
pub struct Outer {
    pub before: ::std::os::raw::c_int,
    pub inner: Inner,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Outer"][::std::mem::size_of::<Outer>() - 32usize];
    ["Alignment of Outer"][::std::mem::align_of::<Outer>() - 16usize];
    ["Offset of field: Outer::before"][::std::mem::offset_of!(Outer, before) - 0usize];
    ["Offset of field: Outer::inner"][::std::mem::offset_of!(Outer, inner) - 16usize];
};
