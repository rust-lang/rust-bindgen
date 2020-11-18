#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bar {
    pub u: foo,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct foo {
    pub foo: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        4usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        4usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<foo>())).foo as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(foo))
    );
}
struct Box_foo {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_foo {}
impl Drop for Box_foo {
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
        unsafe { &(*(::std::ptr::null::<bar>())).u as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(bar), "::", stringify!(u))
    );
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
pub type bar_t = bar;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct baz {
    pub f: foo,
}
#[test]
fn bindgen_test_layout_baz() {
    assert_eq!(
        ::std::mem::size_of::<baz>(),
        4usize,
        concat!("Size of: ", stringify!(baz))
    );
    assert_eq!(
        ::std::mem::align_of::<baz>(),
        4usize,
        concat!("Alignment of ", stringify!(baz))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<baz>())).f as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(baz), "::", stringify!(f))
    );
}
struct Box_baz {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_baz {}
impl Drop for Box_baz {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
