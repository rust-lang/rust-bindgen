#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[repr(align(4))]
#[derive(Default, Copy, Clone)]
pub struct NoDebug {
    pub _bindgen_opaque_blob: u32,
}
const _: () = {
    ["Size of NoDebug"][::std::mem::size_of::<NoDebug>() - 4usize];
    ["Alignment of NoDebug"][::std::mem::align_of::<NoDebug>() - 4usize];
};
