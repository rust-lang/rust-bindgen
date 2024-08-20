#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Copy, Clone)]
pub union color {
    pub u1: color__bindgen_ty_1,
    pub u2: color__bindgen_ty_2,
    pub v3: [::std::os::raw::c_uchar; 3usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct color__bindgen_ty_1 {
    pub r: ::std::os::raw::c_uchar,
    pub g: ::std::os::raw::c_uchar,
    pub b: ::std::os::raw::c_uchar,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of color__bindgen_ty_1",
    ][::std::mem::size_of::<color__bindgen_ty_1>() - 3usize];
    [
        "Alignment of color__bindgen_ty_1",
    ][::std::mem::align_of::<color__bindgen_ty_1>() - 1usize];
    [
        "Offset of field: color__bindgen_ty_1::r",
    ][::std::mem::offset_of!(color__bindgen_ty_1, r) - 0usize];
    [
        "Offset of field: color__bindgen_ty_1::g",
    ][::std::mem::offset_of!(color__bindgen_ty_1, g) - 1usize];
    [
        "Offset of field: color__bindgen_ty_1::b",
    ][::std::mem::offset_of!(color__bindgen_ty_1, b) - 2usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct color__bindgen_ty_2 {
    pub y: ::std::os::raw::c_uchar,
    pub u: ::std::os::raw::c_uchar,
    pub v: ::std::os::raw::c_uchar,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of color__bindgen_ty_2",
    ][::std::mem::size_of::<color__bindgen_ty_2>() - 3usize];
    [
        "Alignment of color__bindgen_ty_2",
    ][::std::mem::align_of::<color__bindgen_ty_2>() - 1usize];
    [
        "Offset of field: color__bindgen_ty_2::y",
    ][::std::mem::offset_of!(color__bindgen_ty_2, y) - 0usize];
    [
        "Offset of field: color__bindgen_ty_2::u",
    ][::std::mem::offset_of!(color__bindgen_ty_2, u) - 1usize];
    [
        "Offset of field: color__bindgen_ty_2::v",
    ][::std::mem::offset_of!(color__bindgen_ty_2, v) - 2usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of color"][::std::mem::size_of::<color>() - 3usize];
    ["Alignment of color"][::std::mem::align_of::<color>() - 1usize];
    ["Offset of field: color::v3"][::std::mem::offset_of!(color, v3) - 0usize];
};
impl Default for color {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
