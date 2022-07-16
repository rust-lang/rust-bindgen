#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Bar {
    pub b: *mut a,
}
#[test]
fn bindgen_test_layout_Bar() {
    const UNINIT: ::std::mem::MaybeUninit<Bar> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Bar>(),
        8usize,
        concat!("Size of: ", stringify!(Bar))
    );
    assert_eq!(
        ::std::mem::align_of::<Bar>(),
        8usize,
        concat!("Alignment of ", stringify!(Bar))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Bar), "::", stringify!(b))
    );
}
impl Default for Bar {
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
pub struct c {
    pub __bindgen_anon_1: c__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union c__bindgen_ty_1 {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_c__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<c__bindgen_ty_1>(),
        1usize,
        concat!("Size of: ", stringify!(c__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<c__bindgen_ty_1>(),
        1usize,
        concat!("Alignment of ", stringify!(c__bindgen_ty_1))
    );
}
impl Default for c__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_c() {
    assert_eq!(
        ::std::mem::size_of::<c>(),
        1usize,
        concat!("Size of: ", stringify!(c))
    );
    assert_eq!(
        ::std::mem::align_of::<c>(),
        1usize,
        concat!("Alignment of ", stringify!(c))
    );
}
impl Default for c {
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
pub struct a {
    pub d: c,
}
#[test]
fn bindgen_test_layout_a() {
    const UNINIT: ::std::mem::MaybeUninit<a> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<a>(),
        1usize,
        concat!("Size of: ", stringify!(a))
    );
    assert_eq!(
        ::std::mem::align_of::<a>(),
        1usize,
        concat!("Alignment of ", stringify!(a))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).d) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(a), "::", stringify!(d))
    );
}
impl Default for a {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
