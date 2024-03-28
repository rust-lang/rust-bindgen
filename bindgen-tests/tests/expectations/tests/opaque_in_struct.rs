#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// <div rustbindgen opaque>
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct opaque {
    pub _bindgen_opaque_blob: u32,
}
const _: () = {
    assert!(::std::mem::size_of::<opaque>() == 4usize, "Size of opaque");
    assert!(::std::mem::align_of::<opaque>() == 4usize, "Alignment of opaque");
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct container {
    pub contained: opaque,
}
const _: () = {
    assert!(::std::mem::size_of::<container>() == 4usize, "Size of container");
    assert!(::std::mem::align_of::<container>() == 4usize, "Alignment of container");
    assert!(
        ::std::mem::offset_of!(container, contained) == 0usize,
        "Offset of field: container::contained",
    );
};
