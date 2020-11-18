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
        unsafe { &(*(::std::ptr::null::<Bar>())).b as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Bar), "::", stringify!(b))
    );
}
impl Default for Bar {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_Bar {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Bar {}
impl Drop for Box_Bar {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(8usize, 8usize).unwrap(),
            );
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
    _bindgen_union_align: u8,
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
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_c__bindgen_ty_1 {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_c__bindgen_ty_1 {}
impl Drop for Box_c__bindgen_ty_1 {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
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
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_c {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_c {}
impl Drop for Box_c {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
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
        unsafe { &(*(::std::ptr::null::<a>())).d as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(a), "::", stringify!(d))
    );
}
impl Default for a {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_a {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_a {}
impl Drop for Box_a {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
        }
    }
}
