#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![no_std]
mod libc {
    pub type c_int = i32;
    pub enum c_void {}
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct foo {
    pub a: libc::c_int,
    pub b: libc::c_int,
    pub bar: *mut libc::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of foo"][::core::mem::size_of::<foo>() - 16usize];
    ["Alignment of foo"][::core::mem::align_of::<foo>() - 8usize];
    ["Offset of field: foo::a"][::core::mem::offset_of!(foo, a) - 0usize];
    ["Offset of field: foo::b"][::core::mem::offset_of!(foo, b) - 4usize];
    ["Offset of field: foo::bar"][::core::mem::offset_of!(foo, bar) - 8usize];
};
impl Default for foo {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
