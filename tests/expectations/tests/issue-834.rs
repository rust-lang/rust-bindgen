#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct U {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_U() {
    assert_eq!(
        ::std::mem::size_of::<U>(),
        1usize,
        concat!("Size of: ", stringify!(U))
    );
    assert_eq!(
        ::std::mem::align_of::<U>(),
        1usize,
        concat!("Alignment of ", stringify!(U))
    );
}
struct Box_U {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_U {}
impl Drop for Box_U {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
        }
    }
}
