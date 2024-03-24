#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// <div rustbindgen opaque></div>
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OtherOpaque {
    pub _bindgen_opaque_blob: u32,
}
const _: () = {
    ["Size of OtherOpaque"][::std::mem::size_of::<OtherOpaque>() - 4usize];
    ["Alignment of OtherOpaque"][::std::mem::align_of::<OtherOpaque>() - 4usize];
};
/// <div rustbindgen opaque></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Opaque {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq)]
pub struct WithOpaquePtr {
    pub whatever: *mut u8,
    pub other: u32,
    pub t: OtherOpaque,
}
const _: () = {
    ["Size of WithOpaquePtr"][::std::mem::size_of::<WithOpaquePtr>() - 16usize];
    ["Alignment of WithOpaquePtr"][::std::mem::align_of::<WithOpaquePtr>() - 8usize];
    [
        "Offset of field: WithOpaquePtr::whatever",
    ][::std::mem::offset_of!(WithOpaquePtr, whatever) - 0usize];
    [
        "Offset of field: WithOpaquePtr::other",
    ][::std::mem::offset_of!(WithOpaquePtr, other) - 8usize];
    [
        "Offset of field: WithOpaquePtr::t",
    ][::std::mem::offset_of!(WithOpaquePtr, t) - 12usize];
};
impl Default for WithOpaquePtr {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
