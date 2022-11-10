#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct foo {
    pub inner: ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_foo() {
    const UNINIT: ::std::mem::MaybeUninit<foo> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        1usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        1usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inner) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(foo),
            "::",
            stringify!(inner)
        )
    );
}
pub type foo_ptr = *const foo;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bar {
    pub inner: ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_bar() {
    const UNINIT: ::std::mem::MaybeUninit<bar> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<bar>(),
        1usize,
        concat!("Size of: ", stringify!(bar))
    );
    assert_eq!(
        ::std::mem::align_of::<bar>(),
        1usize,
        concat!("Alignment of ", stringify!(bar))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inner) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bar),
            "::",
            stringify!(inner)
        )
    );
}
pub type bar_ptr = *mut bar;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct baz {
    _unused: [u8; 0],
}
pub type baz_ptr = *mut baz;
#[repr(C)]
#[derive(Copy, Clone)]
pub union cat {
    pub standard_issue: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_cat() {
    const UNINIT: ::std::mem::MaybeUninit<cat> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<cat>(),
        4usize,
        concat!("Size of: ", stringify!(cat))
    );
    assert_eq!(
        ::std::mem::align_of::<cat>(),
        4usize,
        concat!("Alignment of ", stringify!(cat))
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).standard_issue) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cat),
            "::",
            stringify!(standard_issue)
        )
    );
}
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
extern "C" {
    pub fn takes_foo_ptr(arg1: foo_ptr);
}
extern "C" {
    pub fn takes_foo_struct(arg1: foo);
}
extern "C" {
    pub fn takes_bar_ptr(arg1: bar_ptr);
}
extern "C" {
    pub fn takes_bar_struct(arg1: bar);
}
extern "C" {
    pub fn takes_baz_ptr(arg1: baz_ptr);
}
extern "C" {
    pub fn takes_baz_struct(arg1: baz);
}
extern "C" {
    pub fn takes_cat_ptr(arg1: cat_ptr);
}
extern "C" {
    pub fn takes_cat_union(arg1: cat);
}
extern "C" {
    pub fn takes_mad_ptr(arg1: mad_ptr);
}
extern "C" {
    pub fn takes_mad_enum(arg1: mad);
}
