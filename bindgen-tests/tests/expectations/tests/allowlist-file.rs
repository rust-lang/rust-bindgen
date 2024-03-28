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
const _: () = {
    assert!(::std::mem::size_of::<someClass>() == 1usize, "Size of someClass");
    assert!(::std::mem::align_of::<someClass>() == 1usize, "Alignment of someClass");
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
const _: () = {
    assert!(
        ::std::mem::size_of::<StructWithAllowlistedDefinition>() == 8usize,
        "Size of StructWithAllowlistedDefinition",
    );
    assert!(
        ::std::mem::align_of::<StructWithAllowlistedDefinition>() == 8usize,
        "Alignment of StructWithAllowlistedDefinition",
    );
    assert!(
        ::std::mem::offset_of!(StructWithAllowlistedDefinition, other) == 0usize,
        "Offset of field: StructWithAllowlistedDefinition::other",
    );
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
const _: () = {
    assert!(
        ::std::mem::size_of::<StructWithAllowlistedFwdDecl>() == 4usize,
        "Size of StructWithAllowlistedFwdDecl",
    );
    assert!(
        ::std::mem::align_of::<StructWithAllowlistedFwdDecl>() == 4usize,
        "Alignment of StructWithAllowlistedFwdDecl",
    );
    assert!(
        ::std::mem::offset_of!(StructWithAllowlistedFwdDecl, b) == 0usize,
        "Offset of field: StructWithAllowlistedFwdDecl::b",
    );
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AllowlistMe {
    pub foo: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<AllowlistMe>() == 4usize, "Size of AllowlistMe");
    assert!(::std::mem::align_of::<AllowlistMe>() == 4usize, "Alignment of AllowlistMe");
    assert!(
        ::std::mem::offset_of!(AllowlistMe, foo) == 0usize,
        "Offset of field: AllowlistMe::foo",
    );
};
