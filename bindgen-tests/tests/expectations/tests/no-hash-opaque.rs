#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct NoHash {
    pub _bindgen_opaque_blob: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of NoHash"][::std::mem::size_of::<NoHash>() - 4usize];
    ["Alignment of NoHash"][::std::mem::align_of::<NoHash>() - 4usize];
};
