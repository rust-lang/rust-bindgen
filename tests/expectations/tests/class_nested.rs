#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct A {
    pub member_a: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct A_B {
    pub member_b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_A_B() {
    assert_eq!(
        ::std::mem::size_of::<A_B>(),
        4usize,
        concat!("Size of: ", stringify!(A_B))
    );
    assert_eq!(
        ::std::mem::align_of::<A_B>(),
        4usize,
        concat!("Alignment of ", stringify!(A_B))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<A_B>())).member_b as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(A_B),
            "::",
            stringify!(member_b)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct A_D<T> {
    pub foo: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for A_D<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
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
        unsafe { &(*(::std::ptr::null::<A>())).member_a as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(A),
            "::",
            stringify!(member_a)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct A_C {
    pub baz: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_A_C() {
    assert_eq!(
        ::std::mem::size_of::<A_C>(),
        4usize,
        concat!("Size of: ", stringify!(A_C))
    );
    assert_eq!(
        ::std::mem::align_of::<A_C>(),
        4usize,
        concat!("Alignment of ", stringify!(A_C))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A_C>())).baz as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(A_C), "::", stringify!(baz))
    );
}
extern "C" {
    pub static mut var: A_B;
}
#[test]
fn __bindgen_test_layout_A_D_open0_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<A_D<::std::os::raw::c_int>>(),
        4usize,
        concat!(
            "Size of template specialization: ",
            stringify!(A_D<::std::os::raw::c_int>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<A_D<::std::os::raw::c_int>>(),
        4usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(A_D<::std::os::raw::c_int>)
        )
    );
}
extern "C" {
    pub static mut baz: A_D<::std::os::raw::c_int>;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct D {
    pub member: A_B,
}
#[test]
fn bindgen_test_layout_D() {
    assert_eq!(
        ::std::mem::size_of::<D>(),
        4usize,
        concat!("Size of: ", stringify!(D))
    );
    assert_eq!(
        ::std::mem::align_of::<D>(),
        4usize,
        concat!("Alignment of ", stringify!(D))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<D>())).member as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(D), "::", stringify!(member))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Templated<T> {
    pub member: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Templated_Templated_inner<T> {
    pub member_ptr: *mut T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for Templated_Templated_inner<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl<T> Default for Templated<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
