#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
pub struct A__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct A {
    pub vtable_: *const A__bindgen_vtable,
    pub member: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_A() {
    const UNINIT: ::std::mem::MaybeUninit<A> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<A>(),
        16usize,
        concat!("Size of: ", stringify!(A))
    );
    assert_eq!(
        ::std::mem::align_of::<A>(),
        8usize,
        concat!("Alignment of ", stringify!(A))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).member) as usize - ptr as usize },
        8usize,
        concat!("Offset of field: ", stringify!(A), "::", stringify!(member))
    );
}
impl Default for A {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct B__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct B {
    pub vtable_: *const B__bindgen_vtable,
    pub member2: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_B() {
    const UNINIT: ::std::mem::MaybeUninit<B> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<B>(),
        16usize,
        concat!("Size of: ", stringify!(B))
    );
    assert_eq!(
        ::std::mem::align_of::<B>(),
        8usize,
        concat!("Alignment of ", stringify!(B))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).member2) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(B),
            "::",
            stringify!(member2)
        )
    );
}
impl Default for B {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct C {
    pub _base: A,
    pub _base_1: B,
    pub member3: f32,
}
#[test]
fn bindgen_test_layout_C() {
    const UNINIT: ::std::mem::MaybeUninit<C> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<C>(),
        40usize,
        concat!("Size of: ", stringify!(C))
    );
    assert_eq!(
        ::std::mem::align_of::<C>(),
        8usize,
        concat!("Alignment of ", stringify!(C))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).member3) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(C),
            "::",
            stringify!(member3)
        )
    );
}
impl Default for C {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
