#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct NoHash {
    pub _bindgen_opaque_blob: u32,
}
#[test]
fn bindgen_test_layout_NoHash() {
    assert_eq!(
        ::std::mem::size_of::<NoHash>(),
        4usize,
        concat!("Size of: ", stringify!(NoHash))
    );
    assert_eq!(
        ::std::mem::align_of::<NoHash>(),
        4usize,
        concat!("Alignment of ", stringify!(NoHash))
    );
}
struct Box_NoHash {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_NoHash {}
impl Drop for Box_NoHash {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
