#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct foo {
    pub mMember: [::std::os::raw::c_longlong; 1usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of foo"][::std::mem::size_of::<foo>() - 8usize];
    ["Alignment of foo"][::std::mem::align_of::<foo>() - 8usize];
    ["Offset of field: foo::mMember"][::std::mem::offset_of!(foo, mMember) - 0usize];
};
pub type __m128 = [f32; 4usize];
pub type __m128d = [f64; 2usize];
pub type __m128i = [::std::os::raw::c_longlong; 2usize];
extern "C" {
    #[link_name = "\u{1}_Z3fooDv2_xDv2_d"]
    pub fn foo(arg1: __m128i, arg2: __m128d) -> __m128;
}
