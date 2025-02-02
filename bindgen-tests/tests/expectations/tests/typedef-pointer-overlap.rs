#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct foo {
    pub inner: ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of foo"][::std::mem::size_of::<foo>() - 1usize];
    ["Alignment of foo"][::std::mem::align_of::<foo>() - 1usize];
    ["Offset of field: foo::inner"][::std::mem::offset_of!(foo, inner) - 0usize];
};
pub type foo_ptr = *const foo;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bar {
    pub inner: ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bar"][::std::mem::size_of::<bar>() - 1usize];
    ["Alignment of bar"][::std::mem::align_of::<bar>() - 1usize];
    ["Offset of field: bar::inner"][::std::mem::offset_of!(bar, inner) - 0usize];
};
pub type bar_ptr = *mut bar;
#[repr(C)]
#[derive(Debug)]
pub struct baz {
    _unused: [u8; 0],
}
pub type baz_ptr = *mut baz;
#[repr(C)]
#[derive(Copy, Clone)]
pub union cat {
    pub standard_issue: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of cat"][::std::mem::size_of::<cat>() - 4usize];
    ["Alignment of cat"][::std::mem::align_of::<cat>() - 4usize];
    [
        "Offset of field: cat::standard_issue",
    ][::std::mem::offset_of!(cat, standard_issue) - 0usize];
};
impl Default for cat {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type cat_ptr = *mut cat;
pub const mad_scientist: mad = 0;
pub type mad = ::std::os::raw::c_uint;
pub type mad_ptr = *mut mad;
unsafe extern "C" {
    pub fn takes_foo_ptr(arg1: foo_ptr);
}
unsafe extern "C" {
    pub fn takes_foo_struct(arg1: foo);
}
unsafe extern "C" {
    pub fn takes_bar_ptr(arg1: bar_ptr);
}
unsafe extern "C" {
    pub fn takes_bar_struct(arg1: bar);
}
unsafe extern "C" {
    pub fn takes_baz_ptr(arg1: baz_ptr);
}
unsafe extern "C" {
    pub fn takes_baz_struct(arg1: baz);
}
unsafe extern "C" {
    pub fn takes_cat_ptr(arg1: cat_ptr);
}
unsafe extern "C" {
    pub fn takes_cat_union(arg1: cat);
}
unsafe extern "C" {
    pub fn takes_mad_ptr(arg1: mad_ptr);
}
unsafe extern "C" {
    pub fn takes_mad_enum(arg1: mad);
}
