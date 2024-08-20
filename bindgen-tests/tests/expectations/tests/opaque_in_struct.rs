#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// <div rustbindgen opaque>
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct opaque {
    pub _bindgen_opaque_blob: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of opaque"][::std::mem::size_of::<opaque>() - 4usize];
    ["Alignment of opaque"][::std::mem::align_of::<opaque>() - 4usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct container {
    pub contained: opaque,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of container"][::std::mem::size_of::<container>() - 4usize];
    ["Alignment of container"][::std::mem::align_of::<container>() - 4usize];
    [
        "Offset of field: container::contained",
    ][::std::mem::offset_of!(container, contained) - 0usize];
};
