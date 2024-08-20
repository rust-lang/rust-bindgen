#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of someClass"][::std::mem::size_of::<someClass>() - 1usize];
    ["Alignment of someClass"][::std::mem::align_of::<someClass>() - 1usize];
};
extern "C" {
    #[link_name = "\u{1}_ZN9someClass16somePublicMethodEi"]
    pub fn someClass_somePublicMethod(this: *mut someClass, foo: ::std::os::raw::c_int);
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of StructWithAllowlistedDefinition",
    ][::std::mem::size_of::<StructWithAllowlistedDefinition>() - 8usize];
    [
        "Alignment of StructWithAllowlistedDefinition",
    ][::std::mem::align_of::<StructWithAllowlistedDefinition>() - 8usize];
    [
        "Offset of field: StructWithAllowlistedDefinition::other",
    ][::std::mem::offset_of!(StructWithAllowlistedDefinition, other) - 0usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of StructWithAllowlistedFwdDecl",
    ][::std::mem::size_of::<StructWithAllowlistedFwdDecl>() - 4usize];
    [
        "Alignment of StructWithAllowlistedFwdDecl",
    ][::std::mem::align_of::<StructWithAllowlistedFwdDecl>() - 4usize];
    [
        "Offset of field: StructWithAllowlistedFwdDecl::b",
    ][::std::mem::offset_of!(StructWithAllowlistedFwdDecl, b) - 0usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AllowlistMe {
    pub foo: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of AllowlistMe"][::std::mem::size_of::<AllowlistMe>() - 4usize];
    ["Alignment of AllowlistMe"][::std::mem::align_of::<AllowlistMe>() - 4usize];
    [
        "Offset of field: AllowlistMe::foo",
    ][::std::mem::offset_of!(AllowlistMe, foo) - 0usize];
};
