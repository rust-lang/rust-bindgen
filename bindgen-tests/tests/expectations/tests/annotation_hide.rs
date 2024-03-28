#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// <div rustbindgen opaque></div>
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct D {
    pub _bindgen_opaque_blob: u32,
}
const _: () = {
    assert!(::std::mem::size_of::<D>() == 4usize, "Size of D");
    assert!(::std::mem::align_of::<D>() == 4usize, "Alignment of D");
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct NotAnnotated {
    pub f: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<NotAnnotated>() == 4usize, "Size of NotAnnotated");
    assert!(
        ::std::mem::align_of::<NotAnnotated>() == 4usize,
        "Alignment of NotAnnotated",
    );
    assert!(
        ::std::mem::offset_of!(NotAnnotated, f) == 0usize,
        "Offset of field: NotAnnotated::f",
    );
};
