#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo {
    pub bar: [foo__bindgen_ty_1; 2usize],
    pub baz: [[[foo__bindgen_ty_2; 4usize]; 3usize]; 2usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_1 {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
}
const _: () = {
    ["Size of foo__bindgen_ty_1"][::std::mem::size_of::<foo__bindgen_ty_1>() - 8usize];
    [
        "Alignment of foo__bindgen_ty_1",
    ][::std::mem::align_of::<foo__bindgen_ty_1>() - 4usize];
    [
        "Offset of field: foo__bindgen_ty_1::a",
    ][::std::mem::offset_of!(foo__bindgen_ty_1, a) - 0usize];
    [
        "Offset of field: foo__bindgen_ty_1::b",
    ][::std::mem::offset_of!(foo__bindgen_ty_1, b) - 4usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_2 {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
}
const _: () = {
    ["Size of foo__bindgen_ty_2"][::std::mem::size_of::<foo__bindgen_ty_2>() - 8usize];
    [
        "Alignment of foo__bindgen_ty_2",
    ][::std::mem::align_of::<foo__bindgen_ty_2>() - 4usize];
    [
        "Offset of field: foo__bindgen_ty_2::a",
    ][::std::mem::offset_of!(foo__bindgen_ty_2, a) - 0usize];
    [
        "Offset of field: foo__bindgen_ty_2::b",
    ][::std::mem::offset_of!(foo__bindgen_ty_2, b) - 4usize];
};
const _: () = {
    ["Size of foo"][::std::mem::size_of::<foo>() - 208usize];
    ["Alignment of foo"][::std::mem::align_of::<foo>() - 4usize];
    ["Offset of field: foo::bar"][::std::mem::offset_of!(foo, bar) - 0usize];
    ["Offset of field: foo::baz"][::std::mem::offset_of!(foo, baz) - 16usize];
};
