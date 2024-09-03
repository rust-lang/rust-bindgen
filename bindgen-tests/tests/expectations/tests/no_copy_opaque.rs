#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default)]
pub struct NoCopy {
    pub _bindgen_opaque_blob: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of NoCopy"][::std::mem::size_of::<NoCopy>() - 4usize];
    ["Alignment of NoCopy"][::std::mem::align_of::<NoCopy>() - 4usize];
};
