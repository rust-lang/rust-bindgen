#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Clone, Default)]
pub struct foo_struct {
    pub inner: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of foo_struct"][::std::mem::size_of::<foo_struct>() - 4usize];
    ["Alignment of foo_struct"][::std::mem::align_of::<foo_struct>() - 4usize];
    [
        "Offset of field: foo_struct::inner",
    ][::std::mem::offset_of!(foo_struct, inner) - 0usize];
};
#[repr(u32)]
#[derive(Clone, Hash, PartialEq, Eq, Copy)]
pub enum foo_enum {
    inner = 0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union foo_union {
    pub fst: ::std::mem::ManuallyDrop<::std::os::raw::c_int>,
    pub snd: ::std::mem::ManuallyDrop<f32>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of foo_union"][::std::mem::size_of::<foo_union>() - 4usize];
    ["Alignment of foo_union"][::std::mem::align_of::<foo_union>() - 4usize];
    ["Offset of field: foo_union::fst"][::std::mem::offset_of!(foo_union, fst) - 0usize];
    ["Offset of field: foo_union::snd"][::std::mem::offset_of!(foo_union, snd) - 0usize];
};
#[repr(C)]
pub struct non_matching {
    pub inner: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of non_matching"][::std::mem::size_of::<non_matching>() - 4usize];
    ["Alignment of non_matching"][::std::mem::align_of::<non_matching>() - 4usize];
    [
        "Offset of field: non_matching::inner",
    ][::std::mem::offset_of!(non_matching, inner) - 0usize];
};
