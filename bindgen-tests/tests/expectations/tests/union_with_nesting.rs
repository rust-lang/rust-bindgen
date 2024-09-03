#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Copy, Clone)]
pub union foo {
    pub a: ::std::os::raw::c_uint,
    pub __bindgen_anon_1: foo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct foo__bindgen_ty_1 {
    pub __bindgen_anon_1: foo__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_2: foo__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union foo__bindgen_ty_1__bindgen_ty_1 {
    pub b1: ::std::os::raw::c_ushort,
    pub b2: ::std::os::raw::c_ushort,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of foo__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::size_of::<foo__bindgen_ty_1__bindgen_ty_1>() - 2usize];
    [
        "Alignment of foo__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::align_of::<foo__bindgen_ty_1__bindgen_ty_1>() - 2usize];
    [
        "Offset of field: foo__bindgen_ty_1__bindgen_ty_1::b1",
    ][::std::mem::offset_of!(foo__bindgen_ty_1__bindgen_ty_1, b1) - 0usize];
    [
        "Offset of field: foo__bindgen_ty_1__bindgen_ty_1::b2",
    ][::std::mem::offset_of!(foo__bindgen_ty_1__bindgen_ty_1, b2) - 0usize];
};
impl Default for foo__bindgen_ty_1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union foo__bindgen_ty_1__bindgen_ty_2 {
    pub c1: ::std::os::raw::c_ushort,
    pub c2: ::std::os::raw::c_ushort,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of foo__bindgen_ty_1__bindgen_ty_2",
    ][::std::mem::size_of::<foo__bindgen_ty_1__bindgen_ty_2>() - 2usize];
    [
        "Alignment of foo__bindgen_ty_1__bindgen_ty_2",
    ][::std::mem::align_of::<foo__bindgen_ty_1__bindgen_ty_2>() - 2usize];
    [
        "Offset of field: foo__bindgen_ty_1__bindgen_ty_2::c1",
    ][::std::mem::offset_of!(foo__bindgen_ty_1__bindgen_ty_2, c1) - 0usize];
    [
        "Offset of field: foo__bindgen_ty_1__bindgen_ty_2::c2",
    ][::std::mem::offset_of!(foo__bindgen_ty_1__bindgen_ty_2, c2) - 0usize];
};
impl Default for foo__bindgen_ty_1__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of foo__bindgen_ty_1"][::std::mem::size_of::<foo__bindgen_ty_1>() - 4usize];
    [
        "Alignment of foo__bindgen_ty_1",
    ][::std::mem::align_of::<foo__bindgen_ty_1>() - 2usize];
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of foo"][::std::mem::size_of::<foo>() - 4usize];
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
