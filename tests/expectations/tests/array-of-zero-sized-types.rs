#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// This should get an `_address` byte.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Empty {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Empty() {
    assert_eq!(
        ::std::mem::size_of::<Empty>(),
        1usize,
        concat!("Size of: ", stringify!(Empty))
    );
    assert_eq!(
        ::std::mem::align_of::<Empty>(),
        1usize,
        concat!("Alignment of ", stringify!(Empty))
    );
}
struct Box_Empty {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Empty {}
impl Drop for Box_Empty {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
        }
    }
}
/// This should not get an `_address` byte, since each `Empty` gets one, meaning
/// that this object is addressable.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct HasArrayOfEmpty {
    pub empties: [Empty; 10usize],
}
#[test]
fn bindgen_test_layout_HasArrayOfEmpty() {
    assert_eq!(
        ::std::mem::size_of::<HasArrayOfEmpty>(),
        10usize,
        concat!("Size of: ", stringify!(HasArrayOfEmpty))
    );
    assert_eq!(
        ::std::mem::align_of::<HasArrayOfEmpty>(),
        1usize,
        concat!("Alignment of ", stringify!(HasArrayOfEmpty))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<HasArrayOfEmpty>())).empties as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(HasArrayOfEmpty),
            "::",
            stringify!(empties)
        )
    );
}
struct Box_HasArrayOfEmpty {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_HasArrayOfEmpty {}
impl Drop for Box_HasArrayOfEmpty {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(10usize, 1usize).unwrap(),
            );
        }
    }
}
