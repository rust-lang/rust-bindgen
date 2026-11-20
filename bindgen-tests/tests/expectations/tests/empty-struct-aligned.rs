#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[repr(align(64))]
#[derive(Debug, Copy, Clone)]
pub struct empty_aligned {}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of empty_aligned"][::std::mem::size_of::<empty_aligned>() - 0usize];
    ["Alignment of empty_aligned"][::std::mem::align_of::<empty_aligned>() - 64usize];
};
impl Default for empty_aligned {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[repr(align(64))]
#[derive(Debug, Copy, Clone)]
pub struct contains_empty_aligned {
    pub before: ::std::os::raw::c_int,
    pub empty: empty_aligned,
    pub after: ::std::os::raw::c_longlong,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of contains_empty_aligned",
    ][::std::mem::size_of::<contains_empty_aligned>() - 128usize];
    [
        "Alignment of contains_empty_aligned",
    ][::std::mem::align_of::<contains_empty_aligned>() - 64usize];
    [
        "Offset of field: contains_empty_aligned::before",
    ][::std::mem::offset_of!(contains_empty_aligned, before) - 0usize];
    [
        "Offset of field: contains_empty_aligned::empty",
    ][::std::mem::offset_of!(contains_empty_aligned, empty) - 64usize];
    [
        "Offset of field: contains_empty_aligned::after",
    ][::std::mem::offset_of!(contains_empty_aligned, after) - 64usize];
};
impl Default for contains_empty_aligned {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
