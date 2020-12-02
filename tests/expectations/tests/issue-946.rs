#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct foo {}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        0usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        1usize,
        concat!("Alignment of ", stringify!(foo))
    );
}
struct Box_foo {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_foo {}
impl Drop for Box_foo {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
        }
    }
}
pub type bar = foo;
