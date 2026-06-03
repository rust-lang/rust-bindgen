#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct non_blocked_my_struct {
    pub a: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of non_blocked_my_struct",
    ][::std::mem::size_of::<non_blocked_my_struct>() - 4usize];
    [
        "Alignment of non_blocked_my_struct",
    ][::std::mem::align_of::<non_blocked_my_struct>() - 4usize];
    [
        "Offset of field: non_blocked_my_struct::a",
    ][::std::mem::offset_of!(non_blocked_my_struct, a) - 0usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub union non_blocked_my_union {
    pub a: ::std::os::raw::c_int,
    pub b: f64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of non_blocked_my_union",
    ][::std::mem::size_of::<non_blocked_my_union>() - 8usize];
    [
        "Alignment of non_blocked_my_union",
    ][::std::mem::align_of::<non_blocked_my_union>() - 8usize];
    [
        "Offset of field: non_blocked_my_union::a",
    ][::std::mem::offset_of!(non_blocked_my_union, a) - 0usize];
    [
        "Offset of field: non_blocked_my_union::b",
    ][::std::mem::offset_of!(non_blocked_my_union, b) - 0usize];
};
impl Default for non_blocked_my_union {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const non_blocked_my_enum_NON_BLOCKED_MY_ENUM_A: non_blocked_my_enum = 0;
pub const non_blocked_my_enum_NON_BLOCKED_MY_ENUM_B: non_blocked_my_enum = 1;
pub type non_blocked_my_enum = ::std::os::raw::c_uint;
pub const non_blocked_my_const: ::std::os::raw::c_int = 10;
