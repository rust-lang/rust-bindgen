#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[repr(align(4))]
pub struct Opaque {
    pub _bindgen_opaque_blob: [u32; 41usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Opaque"][::std::mem::size_of::<Opaque>() - 164usize];
    ["Alignment of Opaque"][::std::mem::align_of::<Opaque>() - 4usize];
};
impl Default for Opaque {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for Opaque {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "Opaque {{ opaque }}")
    }
}
#[repr(C)]
pub struct OpaqueUser {
    pub opaque: Opaque,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OpaqueUser"][::std::mem::size_of::<OpaqueUser>() - 164usize];
    ["Alignment of OpaqueUser"][::std::mem::align_of::<OpaqueUser>() - 4usize];
    [
        "Offset of field: OpaqueUser::opaque",
    ][::std::mem::offset_of!(OpaqueUser, opaque) - 0usize];
};
impl Default for OpaqueUser {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for OpaqueUser {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "OpaqueUser {{ opaque: {:?} }}", self.opaque)
    }
}
