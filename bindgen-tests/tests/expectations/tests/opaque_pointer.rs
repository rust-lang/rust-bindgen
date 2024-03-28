#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// <div rustbindgen opaque></div>
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OtherOpaque {
    pub _bindgen_opaque_blob: u32,
}
const _: () = {
    assert!(::std::mem::size_of::<OtherOpaque>() == 4usize, "Size of OtherOpaque");
    assert!(::std::mem::align_of::<OtherOpaque>() == 4usize, "Alignment of OtherOpaque");
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
    assert!(::std::mem::size_of::<WithOpaquePtr>() == 16usize, "Size of WithOpaquePtr");
    assert!(
        ::std::mem::align_of::<WithOpaquePtr>() == 8usize,
        "Alignment of WithOpaquePtr",
    );
    assert!(
        ::std::mem::offset_of!(WithOpaquePtr, whatever) == 0usize,
        "Offset of field: WithOpaquePtr::whatever",
    );
    assert!(
        ::std::mem::offset_of!(WithOpaquePtr, other) == 8usize,
        "Offset of field: WithOpaquePtr::other",
    );
    assert!(
        ::std::mem::offset_of!(WithOpaquePtr, t) == 12usize,
        "Offset of field: WithOpaquePtr::t",
    );
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
