#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo {
    pub bar: foo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_1 {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
}
const _: () = {
    assert!(
        ::std::mem::size_of::<foo__bindgen_ty_1>() == 8usize,
        "Size of foo__bindgen_ty_1",
    );
    assert!(
        ::std::mem::align_of::<foo__bindgen_ty_1>() == 4usize,
        "Alignment of foo__bindgen_ty_1",
    );
    assert!(
        ::std::mem::offset_of!(foo__bindgen_ty_1, a) == 0usize,
        "Offset of field: foo__bindgen_ty_1::a",
    );
    assert!(
        ::std::mem::offset_of!(foo__bindgen_ty_1, b) == 4usize,
        "Offset of field: foo__bindgen_ty_1::b",
    );
};
const _: () = {
    assert!(::std::mem::size_of::<foo>() == 8usize, "Size of foo");
    assert!(::std::mem::align_of::<foo>() == 4usize, "Alignment of foo");
    assert!(::std::mem::offset_of!(foo, bar) == 0usize, "Offset of field: foo::bar");
};
