#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type size_t = ::std::os::raw::c_ulong;
pub type ssize_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct A {
    pub len: size_t,
    pub offset: ssize_t,
    pub next: *mut A,
}
const _: () = {
    assert!(::std::mem::size_of::<A>() == 24usize, "Size of A");
    assert!(::std::mem::align_of::<A>() == 8usize, "Alignment of A");
    assert!(::std::mem::offset_of!(A, len) == 0usize, "Offset of field: A::len");
    assert!(::std::mem::offset_of!(A, offset) == 8usize, "Offset of field: A::offset");
    assert!(::std::mem::offset_of!(A, next) == 16usize, "Offset of field: A::next");
};
impl Default for A {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
