#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct struct_a {
    pub a: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<struct_a>() == 4usize, "Size of struct_a");
    assert!(::std::mem::align_of::<struct_a>() == 4usize, "Alignment of struct_a");
    assert!(
        ::std::mem::offset_of!(struct_a, a) == 0usize,
        "Offset of field: struct_a::a",
    );
};
pub type a = *const struct_a;
#[repr(C)]
#[derive(Copy, Clone)]
pub union union_b {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<union_b>() == 4usize, "Size of union_b");
    assert!(::std::mem::align_of::<union_b>() == 4usize, "Alignment of union_b");
    assert!(::std::mem::offset_of!(union_b, a) == 0usize, "Offset of field: union_b::a");
    assert!(::std::mem::offset_of!(union_b, b) == 0usize, "Offset of field: union_b::b");
};
impl Default for union_b {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type b = union_b;
pub const enum_c_A: enum_c = 0;
pub type enum_c = ::std::os::raw::c_uint;
extern "C" {
    pub fn takes_a(arg: a);
}
extern "C" {
    pub fn takes_b(arg: b);
}
extern "C" {
    pub fn takes_c(arg: enum_c);
}
