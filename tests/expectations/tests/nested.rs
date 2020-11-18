#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Calc {
    pub w: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Calc() {
    assert_eq!(
        ::std::mem::size_of::<Calc>(),
        4usize,
        concat!("Size of: ", stringify!(Calc))
    );
    assert_eq!(
        ::std::mem::align_of::<Calc>(),
        4usize,
        concat!("Alignment of ", stringify!(Calc))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Calc>())).w as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Calc), "::", stringify!(w))
    );
}
struct Box_Calc {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Calc {}
impl Drop for Box_Calc {
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
#[derive(Debug, Default, Copy, Clone)]
pub struct Test {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Test_Size {
    pub mWidth: Test_Size_Dimension,
    pub mHeight: Test_Size_Dimension,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Test_Size_Dimension {
    pub _base: Calc,
}
#[test]
fn bindgen_test_layout_Test_Size_Dimension() {
    assert_eq!(
        ::std::mem::size_of::<Test_Size_Dimension>(),
        4usize,
        concat!("Size of: ", stringify!(Test_Size_Dimension))
    );
    assert_eq!(
        ::std::mem::align_of::<Test_Size_Dimension>(),
        4usize,
        concat!("Alignment of ", stringify!(Test_Size_Dimension))
    );
}
struct Box_Test_Size_Dimension {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Test_Size_Dimension {}
impl Drop for Box_Test_Size_Dimension {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
#[test]
fn bindgen_test_layout_Test_Size() {
    assert_eq!(
        ::std::mem::size_of::<Test_Size>(),
        8usize,
        concat!("Size of: ", stringify!(Test_Size))
    );
    assert_eq!(
        ::std::mem::align_of::<Test_Size>(),
        4usize,
        concat!("Alignment of ", stringify!(Test_Size))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Test_Size>())).mWidth as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Test_Size),
            "::",
            stringify!(mWidth)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Test_Size>())).mHeight as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Test_Size),
            "::",
            stringify!(mHeight)
        )
    );
}
struct Box_Test_Size {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Test_Size {}
impl Drop for Box_Test_Size {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(8usize, 4usize).unwrap(),
            );
        }
    }
}
#[test]
fn bindgen_test_layout_Test() {
    assert_eq!(
        ::std::mem::size_of::<Test>(),
        1usize,
        concat!("Size of: ", stringify!(Test))
    );
    assert_eq!(
        ::std::mem::align_of::<Test>(),
        1usize,
        concat!("Alignment of ", stringify!(Test))
    );
}
struct Box_Test {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Test {}
impl Drop for Box_Test {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
        }
    }
}
