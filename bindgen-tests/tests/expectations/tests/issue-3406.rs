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
pub type AlignedInt = ::std::os::raw::c_int;
pub type NestedAlignedInt = AlignedInt;
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
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
pub struct Outer2 {
    pub before: ::std::os::raw::c_int,
    pub __bindgen_padding_0: [u8; 12usize],
    pub one: AlignedInt,
    pub __bindgen_padding_1: [u8; 12usize],
    pub two: AlignedInt,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Outer2"][::std::mem::size_of::<Outer2>() - 48usize];
    ["Alignment of Outer2"][::std::mem::align_of::<Outer2>() - 16usize];
    ["Offset of field: Outer2::before"][::std::mem::offset_of!(Outer2, before) - 0usize];
    ["Offset of field: Outer2::one"][::std::mem::offset_of!(Outer2, one) - 16usize];
    ["Offset of field: Outer2::two"][::std::mem::offset_of!(Outer2, two) - 32usize];
};
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
pub struct Outer3 {
    pub before: ::std::os::raw::c_int,
    pub __bindgen_padding_0: [u8; 12usize],
    pub inner: AlignedInt,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Outer3"][::std::mem::size_of::<Outer3>() - 32usize];
    ["Alignment of Outer3"][::std::mem::align_of::<Outer3>() - 16usize];
    ["Offset of field: Outer3::before"][::std::mem::offset_of!(Outer3, before) - 0usize];
    ["Offset of field: Outer3::inner"][::std::mem::offset_of!(Outer3, inner) - 16usize];
};
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
pub struct Outer4 {
    pub before: ::std::os::raw::c_int,
    pub __bindgen_padding_0: [u8; 12usize],
    pub inner: NestedAlignedInt,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Outer4"][::std::mem::size_of::<Outer4>() - 32usize];
    ["Alignment of Outer4"][::std::mem::align_of::<Outer4>() - 16usize];
    ["Offset of field: Outer4::before"][::std::mem::offset_of!(Outer4, before) - 0usize];
    ["Offset of field: Outer4::inner"][::std::mem::offset_of!(Outer4, inner) - 16usize];
};
