#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct _bindgen_ty_1 {
    pub _bindgen_align: [u64; 0],
    pub _bindgen_opaque_blob: [u64; 10usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _bindgen_ty_1"][::std::mem::size_of::<_bindgen_ty_1>() - 80usize];
    ["Alignment of _bindgen_ty_1"][::std::mem::align_of::<_bindgen_ty_1>() - 8usize];
};
unsafe extern "C" {
    pub static mut a: _bindgen_ty_1;
}
