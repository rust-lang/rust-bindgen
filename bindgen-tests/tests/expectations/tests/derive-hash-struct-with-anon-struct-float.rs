#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// A struct containing a struct containing a float that cannot derive Hash/Eq/Ord but can derive PartialEq/PartialOrd
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct foo {
    pub bar: foo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct foo__bindgen_ty_1 {
    pub a: f32,
    pub b: f32,
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
