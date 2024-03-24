#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct a {
    pub val_a: *mut b,
}
const _: () = {
    ["Size of a"][::std::mem::size_of::<a>() - 8usize];
    ["Alignment of a"][::std::mem::align_of::<a>() - 8usize];
    ["Offset of field: a::val_a"][::std::mem::offset_of!(a, val_a) - 0usize];
};
impl Default for a {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct b {
    pub val_b: ::std::os::raw::c_int,
}
const _: () = {
    ["Size of b"][::std::mem::size_of::<b>() - 4usize];
    ["Alignment of b"][::std::mem::align_of::<b>() - 4usize];
    ["Offset of field: b::val_b"][::std::mem::offset_of!(b, val_b) - 0usize];
};
