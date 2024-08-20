#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub a: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of A"][::std::mem::size_of::<A>() - 4usize];
    ["Alignment of A"][::std::mem::align_of::<A>() - 4usize];
    ["Offset of field: A::a"][::std::mem::offset_of!(A, a) - 0usize];
};
extern "C" {
    #[link_name = "\u{1}_ZN1A4makeEv"]
    pub fn make() -> A;
}
impl A {
    #[inline]
    pub unsafe fn make() -> A {
        make()
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct B {
    pub b: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of B"][::std::mem::size_of::<B>() - 4usize];
    ["Alignment of B"][::std::mem::align_of::<B>() - 4usize];
    ["Offset of field: B::b"][::std::mem::offset_of!(B, b) - 0usize];
};
extern "C" {
    #[link_name = "\u{1}_ZN1B4makeEv"]
    pub fn make1() -> B;
}
impl B {
    #[inline]
    pub unsafe fn make() -> B {
        make1()
    }
}
