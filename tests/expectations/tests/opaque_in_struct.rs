#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// <div rustbindgen opaque>
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct opaque {
    pub _bindgen_opaque_blob: u32,
}
#[test]
fn bindgen_test_layout_opaque() {
    assert_eq!(
        ::std::mem::size_of::<opaque>(),
        4usize,
        concat!("Size of: ", stringify!(opaque))
    );
    assert_eq!(
        ::std::mem::align_of::<opaque>(),
        4usize,
        concat!("Alignment of ", stringify!(opaque))
    );
}
struct Box_opaque {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_opaque {}
impl Drop for Box_opaque {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct container {
    pub contained: opaque,
}
#[test]
fn bindgen_test_layout_container() {
    assert_eq!(
        ::std::mem::size_of::<container>(),
        4usize,
        concat!("Size of: ", stringify!(container))
    );
    assert_eq!(
        ::std::mem::align_of::<container>(),
        4usize,
        concat!("Alignment of ", stringify!(container))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<container>())).contained as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(container),
            "::",
            stringify!(contained)
        )
    );
}
struct Box_container {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_container {}
impl Drop for Box_container {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
