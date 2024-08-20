#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct NoDefault {
    pub _bindgen_opaque_blob: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of NoDefault"][::std::mem::size_of::<NoDefault>() - 4usize];
    ["Alignment of NoDefault"][::std::mem::align_of::<NoDefault>() - 4usize];
};
