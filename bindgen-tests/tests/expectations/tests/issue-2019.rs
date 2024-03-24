#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub a: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<A>() == 4usize, "Size of A");
    assert!(::std::mem::align_of::<A>() == 4usize, "Alignment of A");
    assert!(::std::mem::offset_of!(A, a) == 0usize, "Offset of field: A::a");
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
const _: () = {
    assert!(::std::mem::size_of::<B>() == 4usize, "Size of B");
    assert!(::std::mem::align_of::<B>() == 4usize, "Alignment of B");
    assert!(::std::mem::offset_of!(B, b) == 0usize, "Offset of field: B::b");
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
