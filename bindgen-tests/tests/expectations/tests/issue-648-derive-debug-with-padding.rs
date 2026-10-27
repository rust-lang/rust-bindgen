#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// We emit a `[u8; 63usize]` padding field for this struct, which cannot derive
/// Debug/Hash because 63 is over the hard coded limit.
#[repr(C)]
#[repr(align(64))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct NoDebug {
    pub c: ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of NoDebug"][::std::mem::size_of::<NoDebug>() - 64usize];
    ["Alignment of NoDebug"][::std::mem::align_of::<NoDebug>() - 64usize];
    ["Offset of field: NoDebug::c"][::std::mem::offset_of!(NoDebug, c) - 0usize];
};
impl Default for NoDebug {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// This should derive Debug/Hash/PartialEq/Eq because the padding size is less than the max derive
/// Debug/Hash/PartialEq/Eq impl for arrays. However, we conservatively don't derive Debug/Hash because
/// we determine Debug derive-ability before we compute padding, which happens at
/// codegen.
#[repr(C)]
#[repr(align(64))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ShouldDeriveDebugButDoesNot {
    pub c: [::std::os::raw::c_char; 32usize],
    pub d: ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ShouldDeriveDebugButDoesNot",
    ][::std::mem::size_of::<ShouldDeriveDebugButDoesNot>() - 64usize];
    [
        "Alignment of ShouldDeriveDebugButDoesNot",
    ][::std::mem::align_of::<ShouldDeriveDebugButDoesNot>() - 64usize];
    [
        "Offset of field: ShouldDeriveDebugButDoesNot::c",
    ][::std::mem::offset_of!(ShouldDeriveDebugButDoesNot, c) - 0usize];
    [
        "Offset of field: ShouldDeriveDebugButDoesNot::d",
    ][::std::mem::offset_of!(ShouldDeriveDebugButDoesNot, d) - 32usize];
};
impl Default for ShouldDeriveDebugButDoesNot {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
