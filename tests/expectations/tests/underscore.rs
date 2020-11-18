#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub const __: ::std::os::raw::c_int = 10;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ptr_t {
    pub __: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout_ptr_t() {
    assert_eq!(
        ::std::mem::size_of::<ptr_t>(),
        8usize,
        concat!("Size of: ", stringify!(ptr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ptr_t>(),
        1usize,
        concat!("Alignment of ", stringify!(ptr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ptr_t>())).__ as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(ptr_t), "::", stringify!(__))
    );
}
struct Box_ptr_t {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_ptr_t {}
impl Drop for Box_ptr_t {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(8usize, 1usize).unwrap(),
            );
        }
    }
}
