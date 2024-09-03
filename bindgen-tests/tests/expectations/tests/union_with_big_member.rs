#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Copy, Clone)]
pub union WithBigArray {
    pub a: ::std::os::raw::c_int,
    pub b: [::std::os::raw::c_int; 33usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of WithBigArray"][::std::mem::size_of::<WithBigArray>() - 132usize];
    ["Alignment of WithBigArray"][::std::mem::align_of::<WithBigArray>() - 4usize];
    [
        "Offset of field: WithBigArray::a",
    ][::std::mem::offset_of!(WithBigArray, a) - 0usize];
    [
        "Offset of field: WithBigArray::b",
    ][::std::mem::offset_of!(WithBigArray, b) - 0usize];
};
impl Default for WithBigArray {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union WithBigArray2 {
    pub a: ::std::os::raw::c_int,
    pub b: [::std::os::raw::c_char; 33usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of WithBigArray2"][::std::mem::size_of::<WithBigArray2>() - 36usize];
    ["Alignment of WithBigArray2"][::std::mem::align_of::<WithBigArray2>() - 4usize];
    [
        "Offset of field: WithBigArray2::a",
    ][::std::mem::offset_of!(WithBigArray2, a) - 0usize];
    [
        "Offset of field: WithBigArray2::b",
    ][::std::mem::offset_of!(WithBigArray2, b) - 0usize];
};
impl Default for WithBigArray2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union WithBigMember {
    pub a: ::std::os::raw::c_int,
    pub b: WithBigArray,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of WithBigMember"][::std::mem::size_of::<WithBigMember>() - 132usize];
    ["Alignment of WithBigMember"][::std::mem::align_of::<WithBigMember>() - 4usize];
    [
        "Offset of field: WithBigMember::a",
    ][::std::mem::offset_of!(WithBigMember, a) - 0usize];
    [
        "Offset of field: WithBigMember::b",
    ][::std::mem::offset_of!(WithBigMember, b) - 0usize];
};
impl Default for WithBigMember {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
