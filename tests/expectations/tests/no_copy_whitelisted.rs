#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default)]
pub struct NoCopy {
    pub i: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_NoCopy() {
    assert_eq!(
        ::std::mem::size_of::<NoCopy>(),
        4usize,
        concat!("Size of: ", stringify!(NoCopy))
    );
    assert_eq!(
        ::std::mem::align_of::<NoCopy>(),
        4usize,
        concat!("Alignment of ", stringify!(NoCopy))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<NoCopy>())).i as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(NoCopy), "::", stringify!(i))
    );
}
struct Box_NoCopy {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_NoCopy {}
impl Drop for Box_NoCopy {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
