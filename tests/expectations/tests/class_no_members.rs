#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct whatever {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_whatever() {
    assert_eq!(
        ::std::mem::size_of::<whatever>(),
        1usize,
        concat!("Size of: ", stringify!(whatever))
    );
    assert_eq!(
        ::std::mem::align_of::<whatever>(),
        1usize,
        concat!("Alignment of ", stringify!(whatever))
    );
}
struct Box_whatever {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_whatever {}
impl Drop for Box_whatever {
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
pub struct whatever_child {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_whatever_child() {
    assert_eq!(
        ::std::mem::size_of::<whatever_child>(),
        1usize,
        concat!("Size of: ", stringify!(whatever_child))
    );
    assert_eq!(
        ::std::mem::align_of::<whatever_child>(),
        1usize,
        concat!("Alignment of ", stringify!(whatever_child))
    );
}
struct Box_whatever_child {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_whatever_child {}
impl Drop for Box_whatever_child {
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
pub struct whatever_child_with_member {
    pub m_member: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_whatever_child_with_member() {
    assert_eq!(
        ::std::mem::size_of::<whatever_child_with_member>(),
        4usize,
        concat!("Size of: ", stringify!(whatever_child_with_member))
    );
    assert_eq!(
        ::std::mem::align_of::<whatever_child_with_member>(),
        4usize,
        concat!("Alignment of ", stringify!(whatever_child_with_member))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<whatever_child_with_member>())).m_member
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(whatever_child_with_member),
            "::",
            stringify!(m_member)
        )
    );
}
struct Box_whatever_child_with_member {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_whatever_child_with_member {}
impl Drop for Box_whatever_child_with_member {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
