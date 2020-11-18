#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern "C" {
    #[link_name = "\u{1}_Z3fooP9Container"]
    pub fn foo(c: *mut Container);
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Container {
    pub _bindgen_opaque_blob: [u32; 2usize],
}
#[test]
fn bindgen_test_layout_Container() {
    assert_eq!(
        ::std::mem::size_of::<Container>(),
        8usize,
        concat!("Size of: ", stringify!(Container))
    );
    assert_eq!(
        ::std::mem::align_of::<Container>(),
        4usize,
        concat!("Alignment of ", stringify!(Container))
    );
}
struct Box_Container {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Container {}
impl Drop for Box_Container {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(8usize, 4usize).unwrap(),
            );
        }
    }
}
