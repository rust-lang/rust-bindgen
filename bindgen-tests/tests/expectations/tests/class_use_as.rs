#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// <div rustbindgen="true" replaces="whatever"></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct whatever {
    pub replacement: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of whatever"][::std::mem::size_of::<whatever>() - 4usize];
    ["Alignment of whatever"][::std::mem::align_of::<whatever>() - 4usize];
    [
        "Offset of field: whatever::replacement",
    ][::std::mem::offset_of!(whatever, replacement) - 0usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct container {
    pub c: whatever,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of container"][::std::mem::size_of::<container>() - 4usize];
    ["Alignment of container"][::std::mem::align_of::<container>() - 4usize];
    ["Offset of field: container::c"][::std::mem::offset_of!(container, c) - 0usize];
};
