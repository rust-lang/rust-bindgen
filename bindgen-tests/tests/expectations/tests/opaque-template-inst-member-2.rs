#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// This is like `opaque-template-inst-member.hpp` except exercising the cases
/// where we are OK to derive Debug/Hash/PartialEq.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OpaqueTemplate {
    pub _address: u8,
}
/// Should derive Debug/Hash/PartialEq.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ContainsOpaqueTemplate {
    pub mBlah: u32,
    pub mBaz: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ContainsOpaqueTemplate",
    ][::std::mem::size_of::<ContainsOpaqueTemplate>() - 8usize];
    [
        "Alignment of ContainsOpaqueTemplate",
    ][::std::mem::align_of::<ContainsOpaqueTemplate>() - 4usize];
    [
        "Offset of field: ContainsOpaqueTemplate::mBlah",
    ][::std::mem::offset_of!(ContainsOpaqueTemplate, mBlah) - 0usize];
    [
        "Offset of field: ContainsOpaqueTemplate::mBaz",
    ][::std::mem::offset_of!(ContainsOpaqueTemplate, mBaz) - 4usize];
};
/// Should also derive Debug/Hash/PartialEq.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct InheritsOpaqueTemplate {
    pub _base: u8,
    pub wow: *mut ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of InheritsOpaqueTemplate",
    ][::std::mem::size_of::<InheritsOpaqueTemplate>() - 16usize];
    [
        "Alignment of InheritsOpaqueTemplate",
    ][::std::mem::align_of::<InheritsOpaqueTemplate>() - 8usize];
    [
        "Offset of field: InheritsOpaqueTemplate::wow",
    ][::std::mem::offset_of!(InheritsOpaqueTemplate, wow) - 8usize];
};
impl Default for InheritsOpaqueTemplate {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
