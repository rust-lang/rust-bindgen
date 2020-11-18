#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub mod foo {
    pub type Type = ::std::os::raw::c_uint;
    pub const Type: Type = 0;
    pub const Type_: Type = 1;
    pub const Type1: Type = 2;
    pub const Type__: Type = 3;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bar {
    pub member: foo::Type,
}
#[test]
fn bindgen_test_layout_bar() {
    assert_eq!(
        ::std::mem::size_of::<bar>(),
        4usize,
        concat!("Size of: ", stringify!(bar))
    );
    assert_eq!(
        ::std::mem::align_of::<bar>(),
        4usize,
        concat!("Alignment of ", stringify!(bar))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bar>())).member as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bar),
            "::",
            stringify!(member)
        )
    );
}
impl Default for bar {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_bar {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_bar {}
impl Drop for Box_bar {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
