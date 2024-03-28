#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![no_std]
mod libc {
    pub mod foo {
        pub type c_int = i32;
        pub enum c_void {}
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct foo {
    pub a: libc::foo::c_int,
    pub b: libc::foo::c_int,
    pub bar: *mut libc::foo::c_void,
}
const _: () = {
    assert!(::core::mem::size_of::<foo>() == 16usize, "Size of foo");
    assert!(::core::mem::align_of::<foo>() == 8usize, "Alignment of foo");
    assert!(::core::mem::offset_of!(foo, a) == 0usize, "Offset of field: foo::a");
    assert!(::core::mem::offset_of!(foo, b) == 4usize, "Offset of field: foo::b");
    assert!(::core::mem::offset_of!(foo, bar) == 8usize, "Offset of field: foo::bar");
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
