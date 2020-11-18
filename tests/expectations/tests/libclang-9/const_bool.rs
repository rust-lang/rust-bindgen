#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub const k: bool = true;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub _address: u8,
}
pub const A_k: bool = false;
#[test]
fn bindgen_test_layout_A() {
    assert_eq!(
        ::std::mem::size_of::<A>(),
        1usize,
        concat!("Size of: ", stringify!(A))
    );
    assert_eq!(
        ::std::mem::align_of::<A>(),
        1usize,
        concat!("Alignment of ", stringify!(A))
    );
}
struct Box_A {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_A {}
impl Drop for Box_A {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
        }
    }
}
pub type foo = bool;
pub const k2: foo = true;
