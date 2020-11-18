#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone)]
pub union WithBigArray {
    pub a: ::std::os::raw::c_int,
    pub b: [::std::os::raw::c_int; 33usize],
    _bindgen_union_align: [u32; 33usize],
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
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WithBigArray>())).b as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithBigArray),
            "::",
            stringify!(b)
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
#[repr(C)]
#[derive(Copy, Clone)]
pub union WithBigArray2 {
    pub a: ::std::os::raw::c_int,
    pub b: [::std::os::raw::c_char; 33usize],
    _bindgen_union_align: [u32; 9usize],
}
#[test]
fn bindgen_test_layout_WithBigArray2() {
    assert_eq!(
        ::std::mem::size_of::<WithBigArray2>(),
        36usize,
        concat!("Size of: ", stringify!(WithBigArray2))
    );
    assert_eq!(
        ::std::mem::align_of::<WithBigArray2>(),
        4usize,
        concat!("Alignment of ", stringify!(WithBigArray2))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WithBigArray2>())).a as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithBigArray2),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WithBigArray2>())).b as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithBigArray2),
            "::",
            stringify!(b)
        )
    );
}
impl Default for WithBigArray2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_WithBigArray2 {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_WithBigArray2 {}
impl Drop for Box_WithBigArray2 {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(36usize, 4usize).unwrap(),
            );
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union WithBigMember {
    pub a: ::std::os::raw::c_int,
    pub b: WithBigArray,
    _bindgen_union_align: [u32; 33usize],
}
#[test]
fn bindgen_test_layout_WithBigMember() {
    assert_eq!(
        ::std::mem::size_of::<WithBigMember>(),
        132usize,
        concat!("Size of: ", stringify!(WithBigMember))
    );
    assert_eq!(
        ::std::mem::align_of::<WithBigMember>(),
        4usize,
        concat!("Alignment of ", stringify!(WithBigMember))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WithBigMember>())).a as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithBigMember),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WithBigMember>())).b as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithBigMember),
            "::",
            stringify!(b)
        )
    );
}
impl Default for WithBigMember {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_WithBigMember {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_WithBigMember {}
impl Drop for Box_WithBigMember {
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
