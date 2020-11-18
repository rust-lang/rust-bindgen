#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct NoDebug {
    pub i: ::std::os::raw::c_int,
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
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<NoDebug>())).i as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(NoDebug), "::", stringify!(i))
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
