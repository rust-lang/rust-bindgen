#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub const SOME_DEFUN: u32 = 123;
extern "C" {
    #[link_name = "\u{1}_Z12SomeFunctionv"]
    pub fn SomeFunction();
}
extern "C" {
    pub static mut someVar: ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct someClass {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_someClass() {
    assert_eq!(
        ::std::mem::size_of::<someClass>(),
        1usize,
        concat!("Size of: ", stringify!(someClass))
    );
    assert_eq!(
        ::std::mem::align_of::<someClass>(),
        1usize,
        concat!("Alignment of ", stringify!(someClass))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN9someClass16somePublicMethodEi"]
    pub fn someClass_somePublicMethod(
        this: *mut someClass,
        foo: ::std::os::raw::c_int,
    );
}
impl someClass {
    #[inline]
    pub unsafe fn somePublicMethod(&mut self, foo: ::std::os::raw::c_int) {
        someClass_somePublicMethod(self, foo)
    }
}
extern "C" {
    pub fn ExternFunction();
}
extern "C" {
    #[link_name = "\u{1}_ZN3foo18NamespacedFunctionEv"]
    pub fn foo_NamespacedFunction();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StructWithAllowlistedDefinition {
    pub other: *mut StructWithAllowlistedFwdDecl,
}
#[test]
fn bindgen_test_layout_StructWithAllowlistedDefinition() {
    const UNINIT: ::std::mem::MaybeUninit<StructWithAllowlistedDefinition> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<StructWithAllowlistedDefinition>(),
        8usize,
        concat!("Size of: ", stringify!(StructWithAllowlistedDefinition))
    );
    assert_eq!(
        ::std::mem::align_of::<StructWithAllowlistedDefinition>(),
        8usize,
        concat!("Alignment of ", stringify!(StructWithAllowlistedDefinition))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).other) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(StructWithAllowlistedDefinition),
            "::",
            stringify!(other)
        )
    );
}
impl Default for StructWithAllowlistedDefinition {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct StructWithAllowlistedFwdDecl {
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_StructWithAllowlistedFwdDecl() {
    const UNINIT: ::std::mem::MaybeUninit<StructWithAllowlistedFwdDecl> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<StructWithAllowlistedFwdDecl>(),
        4usize,
        concat!("Size of: ", stringify!(StructWithAllowlistedFwdDecl))
    );
    assert_eq!(
        ::std::mem::align_of::<StructWithAllowlistedFwdDecl>(),
        4usize,
        concat!("Alignment of ", stringify!(StructWithAllowlistedFwdDecl))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(StructWithAllowlistedFwdDecl),
            "::",
            stringify!(b)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AllowlistMe {
    pub foo: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_AllowlistMe() {
    const UNINIT: ::std::mem::MaybeUninit<AllowlistMe> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AllowlistMe>(),
        4usize,
        concat!("Size of: ", stringify!(AllowlistMe))
    );
    assert_eq!(
        ::std::mem::align_of::<AllowlistMe>(),
        4usize,
        concat!("Alignment of ", stringify!(AllowlistMe))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).foo) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AllowlistMe),
            "::",
            stringify!(foo)
        )
    );
}
