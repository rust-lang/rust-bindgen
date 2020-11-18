#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsFoo {
    pub details: [f32; 400usize],
}
#[test]
fn bindgen_test_layout_nsFoo() {
    assert_eq!(
        ::std::mem::size_of::<nsFoo>(),
        1600usize,
        concat!("Size of: ", stringify!(nsFoo))
    );
    assert_eq!(
        ::std::mem::align_of::<nsFoo>(),
        4usize,
        concat!("Alignment of ", stringify!(nsFoo))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nsFoo>())).details as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nsFoo),
            "::",
            stringify!(details)
        )
    );
}
impl Default for nsFoo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_nsFoo {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_nsFoo {}
impl Drop for Box_nsFoo {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1600usize, 4usize)
                    .unwrap(),
            );
        }
    }
}
extern "C" {
    pub static gDetails: nsFoo;
}
