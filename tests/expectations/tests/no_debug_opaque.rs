#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[repr(align(4))]
#[derive(Default, Copy, Clone)]
pub struct NoDebug {
    pub _bindgen_opaque_blob: u32,
}
#[test]
fn bindgen_test_layout_NoDebug() {
    assert_eq!(
        ::std::mem::size_of::<NoDebug>(),
        4usize,
        concat!("Size of: ", stringify!(NoDebug))
    );
    assert_eq!(
        ::std::mem::align_of::<NoDebug>(),
        4usize,
        concat!("Alignment of ", stringify!(NoDebug))
    );
}
struct Box_NoDebug {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_NoDebug {}
impl Drop for Box_NoDebug {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
