#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Template<T> {
    pub member: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for Template<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ContainsInstantiation {
    pub not_opaque: Template<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_ContainsInstantiation() {
    assert_eq!(
        ::std::mem::size_of::<ContainsInstantiation>(),
        1usize,
        concat!("Size of: ", stringify!(ContainsInstantiation))
    );
    assert_eq!(
        ::std::mem::align_of::<ContainsInstantiation>(),
        1usize,
        concat!("Alignment of ", stringify!(ContainsInstantiation))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ContainsInstantiation>())).not_opaque
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ContainsInstantiation),
            "::",
            stringify!(not_opaque)
        )
    );
}
impl Default for ContainsInstantiation {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_ContainsInstantiation {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_ContainsInstantiation {}
impl Drop for Box_ContainsInstantiation {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ContainsOpaqueInstantiation {
    pub opaque: u32,
}
#[test]
fn bindgen_test_layout_ContainsOpaqueInstantiation() {
    assert_eq!(
        ::std::mem::size_of::<ContainsOpaqueInstantiation>(),
        4usize,
        concat!("Size of: ", stringify!(ContainsOpaqueInstantiation))
    );
    assert_eq!(
        ::std::mem::align_of::<ContainsOpaqueInstantiation>(),
        4usize,
        concat!("Alignment of ", stringify!(ContainsOpaqueInstantiation))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ContainsOpaqueInstantiation>())).opaque
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ContainsOpaqueInstantiation),
            "::",
            stringify!(opaque)
        )
    );
}
struct Box_ContainsOpaqueInstantiation {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_ContainsOpaqueInstantiation {}
impl Drop for Box_ContainsOpaqueInstantiation {
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
fn __bindgen_test_layout_Template_open0_char_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<Template<::std::os::raw::c_char>>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(Template<::std::os::raw::c_char>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<Template<::std::os::raw::c_char>>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(Template<::std::os::raw::c_char>)
        )
    );
}
