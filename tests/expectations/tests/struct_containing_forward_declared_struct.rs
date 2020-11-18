#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct a {
    pub val_a: *mut b,
}
#[test]
fn bindgen_test_layout_a() {
    assert_eq!(
        ::std::mem::size_of::<a>(),
        8usize,
        concat!("Size of: ", stringify!(a))
    );
    assert_eq!(
        ::std::mem::align_of::<a>(),
        8usize,
        concat!("Alignment of ", stringify!(a))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<a>())).val_a as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(a), "::", stringify!(val_a))
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
                ::std::alloc::Layout::from_size_align(8usize, 8usize).unwrap(),
            );
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct b {
    pub val_b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_b() {
    assert_eq!(
        ::std::mem::size_of::<b>(),
        4usize,
        concat!("Size of: ", stringify!(b))
    );
    assert_eq!(
        ::std::mem::align_of::<b>(),
        4usize,
        concat!("Alignment of ", stringify!(b))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<b>())).val_b as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(b), "::", stringify!(val_b))
    );
}
struct Box_b {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_b {}
impl Drop for Box_b {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
