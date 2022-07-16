#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub a: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_A() {
    const UNINIT: ::std::mem::MaybeUninit<A> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<A>(),
        4usize,
        concat!("Size of: ", stringify!(A))
    );
    assert_eq!(
        ::std::mem::align_of::<A>(),
        4usize,
        concat!("Alignment of ", stringify!(A))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(A), "::", stringify!(a))
    );
}
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
#[test]
fn bindgen_test_layout_B() {
    const UNINIT: ::std::mem::MaybeUninit<B> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<B>(),
        4usize,
        concat!("Size of: ", stringify!(B))
    );
    assert_eq!(
        ::std::mem::align_of::<B>(),
        4usize,
        concat!("Alignment of ", stringify!(B))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(B), "::", stringify!(b))
    );
}
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
