#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct a {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct _bindgen_ty_1 {
    pub ar: a,
}
const _: () = {
    ["Size of _bindgen_ty_1"][::std::mem::size_of::<_bindgen_ty_1>() - 1usize];
    ["Alignment of _bindgen_ty_1"][::std::mem::align_of::<_bindgen_ty_1>() - 1usize];
    [
        "Offset of field: _bindgen_ty_1::ar",
    ][::std::mem::offset_of!(_bindgen_ty_1, ar) - 0usize];
};
extern "C" {
    pub static mut AutoIdVector: _bindgen_ty_1;
}
const _: () = {
    [
        "Size of template specialization: a_open0_int_close0",
    ][::std::mem::size_of::<a>() - 1usize];
    [
        "Align of template specialization: a_open0_int_close0",
    ][::std::mem::align_of::<a>() - 1usize];
};
