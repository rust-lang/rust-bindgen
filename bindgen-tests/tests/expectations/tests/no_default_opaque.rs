#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct NoDefault {
    pub _bindgen_opaque_blob: u32,
}
const _: () = {
    assert!(::std::mem::size_of::<NoDefault>() == 4usize, "Size of NoDefault");
    assert!(::std::mem::align_of::<NoDefault>() == 4usize, "Alignment of NoDefault");
};
