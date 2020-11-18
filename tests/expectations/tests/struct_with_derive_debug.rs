#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct LittleArray {
    pub a: [::std::os::raw::c_int; 32usize],
}
#[test]
fn bindgen_test_layout_LittleArray() {
    assert_eq!(
        ::std::mem::size_of::<LittleArray>(),
        128usize,
        concat!("Size of: ", stringify!(LittleArray))
    );
    assert_eq!(
        ::std::mem::align_of::<LittleArray>(),
        4usize,
        concat!("Alignment of ", stringify!(LittleArray))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<LittleArray>())).a as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(LittleArray),
            "::",
            stringify!(a)
        )
    );
}
struct Box_LittleArray {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_LittleArray {}
impl Drop for Box_LittleArray {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(128usize, 4usize)
                    .unwrap(),
            );
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BigArray {
    pub a: [::std::os::raw::c_int; 33usize],
}
#[test]
fn bindgen_test_layout_BigArray() {
    assert_eq!(
        ::std::mem::size_of::<BigArray>(),
        132usize,
        concat!("Size of: ", stringify!(BigArray))
    );
    assert_eq!(
        ::std::mem::align_of::<BigArray>(),
        4usize,
        concat!("Alignment of ", stringify!(BigArray))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<BigArray>())).a as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(BigArray),
            "::",
            stringify!(a)
        )
    );
}
impl Default for BigArray {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_BigArray {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_BigArray {}
impl Drop for Box_BigArray {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(132usize, 4usize)
                    .unwrap(),
            );
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct WithLittleArray {
    pub a: LittleArray,
}
#[test]
fn bindgen_test_layout_WithLittleArray() {
    assert_eq!(
        ::std::mem::size_of::<WithLittleArray>(),
        128usize,
        concat!("Size of: ", stringify!(WithLittleArray))
    );
    assert_eq!(
        ::std::mem::align_of::<WithLittleArray>(),
        4usize,
        concat!("Alignment of ", stringify!(WithLittleArray))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WithLittleArray>())).a as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithLittleArray),
            "::",
            stringify!(a)
        )
    );
}
struct Box_WithLittleArray {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_WithLittleArray {}
impl Drop for Box_WithLittleArray {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(128usize, 4usize)
                    .unwrap(),
            );
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WithBigArray {
    pub a: BigArray,
}
#[test]
fn bindgen_test_layout_WithBigArray() {
    assert_eq!(
        ::std::mem::size_of::<WithBigArray>(),
        132usize,
        concat!("Size of: ", stringify!(WithBigArray))
    );
    assert_eq!(
        ::std::mem::align_of::<WithBigArray>(),
        4usize,
        concat!("Alignment of ", stringify!(WithBigArray))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WithBigArray>())).a as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithBigArray),
            "::",
            stringify!(a)
        )
    );
}
impl Default for WithBigArray {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_WithBigArray {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_WithBigArray {}
impl Drop for Box_WithBigArray {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(132usize, 4usize)
                    .unwrap(),
            );
        }
    }
}
