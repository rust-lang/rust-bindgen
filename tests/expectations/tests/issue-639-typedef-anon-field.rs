#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub bar: Foo_Bar,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo_Bar {
    pub abc: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Foo_Bar() {
    assert_eq!(
        ::std::mem::size_of::<Foo_Bar>(),
        4usize,
        concat!("Size of: ", stringify!(Foo_Bar))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo_Bar>(),
        4usize,
        concat!("Alignment of ", stringify!(Foo_Bar))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Foo_Bar>())).abc as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Foo_Bar),
            "::",
            stringify!(abc)
        )
    );
}
struct Box_Foo_Bar {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Foo_Bar {}
impl Drop for Box_Foo_Bar {
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
fn bindgen_test_layout_Foo() {
    assert_eq!(
        ::std::mem::size_of::<Foo>(),
        4usize,
        concat!("Size of: ", stringify!(Foo))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo>(),
        4usize,
        concat!("Alignment of ", stringify!(Foo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Foo>())).bar as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Foo), "::", stringify!(bar))
    );
}
struct Box_Foo {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Foo {}
impl Drop for Box_Foo {
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
pub struct Baz {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Baz_Bar {
    pub abc: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Baz_Bar() {
    assert_eq!(
        ::std::mem::size_of::<Baz_Bar>(),
        4usize,
        concat!("Size of: ", stringify!(Baz_Bar))
    );
    assert_eq!(
        ::std::mem::align_of::<Baz_Bar>(),
        4usize,
        concat!("Alignment of ", stringify!(Baz_Bar))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Baz_Bar>())).abc as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Baz_Bar),
            "::",
            stringify!(abc)
        )
    );
}
struct Box_Baz_Bar {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Baz_Bar {}
impl Drop for Box_Baz_Bar {
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
fn bindgen_test_layout_Baz() {
    assert_eq!(
        ::std::mem::size_of::<Baz>(),
        1usize,
        concat!("Size of: ", stringify!(Baz))
    );
    assert_eq!(
        ::std::mem::align_of::<Baz>(),
        1usize,
        concat!("Alignment of ", stringify!(Baz))
    );
}
struct Box_Baz {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Baz {}
impl Drop for Box_Baz {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
        }
    }
}
