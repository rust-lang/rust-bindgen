#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct foo {
    pub a: ::std::os::raw::c_uint,
    pub __bindgen_anon_1: foo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union foo__bindgen_ty_1 {
    pub b: ::std::os::raw::c_uint,
    pub __bindgen_anon_1: foo__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_2: foo__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_1__bindgen_ty_1 {
    pub c1: ::std::os::raw::c_ushort,
    pub c2: ::std::os::raw::c_ushort,
}
const _: () = {
    [
        "Size of foo__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::size_of::<foo__bindgen_ty_1__bindgen_ty_1>() - 4usize];
    [
        "Alignment of foo__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::align_of::<foo__bindgen_ty_1__bindgen_ty_1>() - 2usize];
    [
        "Offset of field: foo__bindgen_ty_1__bindgen_ty_1::c1",
    ][::std::mem::offset_of!(foo__bindgen_ty_1__bindgen_ty_1, c1) - 0usize];
    [
        "Offset of field: foo__bindgen_ty_1__bindgen_ty_1::c2",
    ][::std::mem::offset_of!(foo__bindgen_ty_1__bindgen_ty_1, c2) - 2usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_1__bindgen_ty_2 {
    pub d1: ::std::os::raw::c_uchar,
    pub d2: ::std::os::raw::c_uchar,
    pub d3: ::std::os::raw::c_uchar,
    pub d4: ::std::os::raw::c_uchar,
}
const _: () = {
    [
        "Size of foo__bindgen_ty_1__bindgen_ty_2",
    ][::std::mem::size_of::<foo__bindgen_ty_1__bindgen_ty_2>() - 4usize];
    [
        "Alignment of foo__bindgen_ty_1__bindgen_ty_2",
    ][::std::mem::align_of::<foo__bindgen_ty_1__bindgen_ty_2>() - 1usize];
    [
        "Offset of field: foo__bindgen_ty_1__bindgen_ty_2::d1",
    ][::std::mem::offset_of!(foo__bindgen_ty_1__bindgen_ty_2, d1) - 0usize];
    [
        "Offset of field: foo__bindgen_ty_1__bindgen_ty_2::d2",
    ][::std::mem::offset_of!(foo__bindgen_ty_1__bindgen_ty_2, d2) - 1usize];
    [
        "Offset of field: foo__bindgen_ty_1__bindgen_ty_2::d3",
    ][::std::mem::offset_of!(foo__bindgen_ty_1__bindgen_ty_2, d3) - 2usize];
    [
        "Offset of field: foo__bindgen_ty_1__bindgen_ty_2::d4",
    ][::std::mem::offset_of!(foo__bindgen_ty_1__bindgen_ty_2, d4) - 3usize];
};
const _: () = {
    ["Size of foo__bindgen_ty_1"][::std::mem::size_of::<foo__bindgen_ty_1>() - 4usize];
    [
        "Alignment of foo__bindgen_ty_1",
    ][::std::mem::align_of::<foo__bindgen_ty_1>() - 4usize];
    [
        "Offset of field: foo__bindgen_ty_1::b",
    ][::std::mem::offset_of!(foo__bindgen_ty_1, b) - 0usize];
};
impl Default for foo__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
const _: () = {
    ["Size of foo"][::std::mem::size_of::<foo>() - 8usize];
    ["Alignment of foo"][::std::mem::align_of::<foo>() - 4usize];
    ["Offset of field: foo::a"][::std::mem::offset_of!(foo, a) - 0usize];
};
impl Default for foo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
