#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
pub struct Instance {
    pub val: [u32; 50usize],
}
#[test]
fn bindgen_test_layout_Instance() {
    assert_eq!(
        ::std::mem::size_of::<Instance>(),
        200usize,
        concat!("Size of: ", stringify!(Instance))
    );
    assert_eq!(
        ::std::mem::align_of::<Instance>(),
        4usize,
        concat!("Alignment of ", stringify!(Instance))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Instance>())).val as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Instance),
            "::",
            stringify!(val)
        )
    );
}
impl Default for Instance {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Instance {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "Instance {{ val: opaque }}")
    }
}
struct Box_Instance {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Instance {}
impl Drop for Box_Instance {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(200usize, 4usize)
                    .unwrap(),
            );
        }
    }
}
