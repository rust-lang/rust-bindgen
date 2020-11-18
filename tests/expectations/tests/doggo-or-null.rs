#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq)]
pub struct Doggo {
    pub x: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Doggo() {
    assert_eq!(
        ::std::mem::size_of::<Doggo>(),
        4usize,
        concat!("Size of: ", stringify!(Doggo))
    );
    assert_eq!(
        ::std::mem::align_of::<Doggo>(),
        4usize,
        concat!("Alignment of ", stringify!(Doggo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Doggo>())).x as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Doggo), "::", stringify!(x))
    );
}
struct Box_Doggo {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Doggo {}
impl Drop for Box_Doggo {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq)]
pub struct Null {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Null() {
    assert_eq!(
        ::std::mem::size_of::<Null>(),
        1usize,
        concat!("Size of: ", stringify!(Null))
    );
    assert_eq!(
        ::std::mem::align_of::<Null>(),
        1usize,
        concat!("Alignment of ", stringify!(Null))
    );
}
struct Box_Null {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Null {}
impl Drop for Box_Null {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
        }
    }
}
/// This type is an opaque union. Unions can't derive anything interesting like
/// Debug or Default, even if their layout can, because it would require knowing
/// which variant is in use. Opaque unions still end up as a `union` in the Rust
/// bindings, but they just have one variant. Even so, can't derive. We should
/// probably emit an opaque struct for opaque unions... but until then, we have
/// this test to make sure that opaque unions don't derive and still compile.
#[repr(C)]
#[repr(align(4))]
#[derive(Copy, Clone)]
pub union DoggoOrNull {
    pub _bindgen_opaque_blob: u32,
}
#[test]
fn bindgen_test_layout_DoggoOrNull() {
    assert_eq!(
        ::std::mem::size_of::<DoggoOrNull>(),
        4usize,
        concat!("Size of: ", stringify!(DoggoOrNull))
    );
    assert_eq!(
        ::std::mem::align_of::<DoggoOrNull>(),
        4usize,
        concat!("Alignment of ", stringify!(DoggoOrNull))
    );
}
impl Default for DoggoOrNull {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_DoggoOrNull {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_DoggoOrNull {}
impl Drop for Box_DoggoOrNull {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
