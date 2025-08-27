#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
extern crate core;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct C {
    pub large_array: [::core::ffi::c_int; 420usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of C"][::core::mem::size_of::<C>() - 1680usize];
    ["Alignment of C"][::core::mem::align_of::<C>() - 4usize];
    [
        "Offset of field: C::large_array",
    ][::core::mem::offset_of!(C, large_array) - 0usize];
};
impl Default for C {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
