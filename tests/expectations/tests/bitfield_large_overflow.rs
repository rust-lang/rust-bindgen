#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct _bindgen_ty_1 {
    pub _bindgen_opaque_blob: [u64; 10usize],
}
#[test]
fn bindgen_test_layout__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_bindgen_ty_1>(),
        80usize,
        concat!("Size of: ", stringify!(_bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<_bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(_bindgen_ty_1))
    );
}
struct Box__bindgen_ty_1 {
    ptr: *mut ::std::ffi::c_void,
}
impl Box__bindgen_ty_1 {}
impl Drop for Box__bindgen_ty_1 {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(80usize, 8usize).unwrap(),
            );
        }
    }
}
extern "C" {
    pub static mut a: _bindgen_ty_1;
}
