#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub foo: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_A() {
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
        unsafe { &(*(::std::ptr::null::<A>())).foo as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(A), "::", stringify!(foo))
    );
}
#[repr(C)]
pub struct B__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct B {
    pub vtable_: *const B__bindgen_vtable,
    pub bar: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_B() {
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
        unsafe { &(*(::std::ptr::null::<B>())).bar as *const _ as usize },
        8usize,
        concat!("Offset of field: ", stringify!(B), "::", stringify!(bar))
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
pub struct C__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct C {
    pub vtable_: *const C__bindgen_vtable,
    pub baz: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(
        ::std::mem::size_of::<C>(),
        16usize,
        concat!("Size of: ", stringify!(C))
    );
    assert_eq!(
        ::std::mem::align_of::<C>(),
        8usize,
        concat!("Alignment of ", stringify!(C))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<C>())).baz as *const _ as usize },
        8usize,
        concat!("Offset of field: ", stringify!(C), "::", stringify!(baz))
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct D {
    pub _base: C,
    pub _base_1: B,
    pub bazz: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_D() {
    assert_eq!(
        ::std::mem::size_of::<D>(),
        40usize,
        concat!("Size of: ", stringify!(D))
    );
    assert_eq!(
        ::std::mem::align_of::<D>(),
        8usize,
        concat!("Alignment of ", stringify!(D))
    );
}
impl Default for D {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
