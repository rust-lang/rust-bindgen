#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// <div rustbindgen opaque></div>
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct D {
    pub _bindgen_opaque_blob: u32,
}
const _: () = {
    ["Size of D"][::std::mem::size_of::<D>() - 4usize];
    ["Alignment of D"][::std::mem::align_of::<D>() - 4usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct NotAnnotated {
    pub f: ::std::os::raw::c_int,
}
const _: () = {
    ["Size of NotAnnotated"][::std::mem::size_of::<NotAnnotated>() - 4usize];
    ["Alignment of NotAnnotated"][::std::mem::align_of::<NotAnnotated>() - 4usize];
    [
        "Offset of field: NotAnnotated::f",
    ][::std::mem::offset_of!(NotAnnotated, f) - 0usize];
};
