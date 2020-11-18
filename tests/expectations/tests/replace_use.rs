#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// <div rustbindgen replaces="nsTArray"></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct nsTArray {
    pub y: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Test {
    pub a: nsTArray,
}
#[test]
fn bindgen_test_layout_Test() {
    assert_eq!(
        ::std::mem::size_of::<Test>(),
        4usize,
        concat!("Size of: ", stringify!(Test))
    );
    assert_eq!(
        ::std::mem::align_of::<Test>(),
        4usize,
        concat!("Alignment of ", stringify!(Test))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Test>())).a as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(a))
    );
}
struct Box_Test {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Test {}
impl Drop for Box_Test {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
#[test]
fn __bindgen_test_layout_nsTArray_open0_long_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<nsTArray>(),
        4usize,
        concat!("Size of template specialization: ", stringify!(nsTArray))
    );
    assert_eq!(
        ::std::mem::align_of::<nsTArray>(),
        4usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(nsTArray)
        )
    );
}
