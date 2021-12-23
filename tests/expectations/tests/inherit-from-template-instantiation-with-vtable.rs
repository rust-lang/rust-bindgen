#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
pub struct BaseWithVtable__bindgen_vtable(::std::os::raw::c_void);
/// This should have an explicit vtable.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BaseWithVtable<T> {
    pub vtable_: *const BaseWithVtable__bindgen_vtable,
    pub t: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for BaseWithVtable<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// This should not have an explicit vtable.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DerivedWithNoVirtualMethods {
    pub _base: BaseWithVtable<*mut ::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_DerivedWithNoVirtualMethods() {
    assert_eq!(
        ::std::mem::size_of::<DerivedWithNoVirtualMethods>(),
        16usize,
        concat!("Size of: ", stringify!(DerivedWithNoVirtualMethods))
    );
    assert_eq!(
        ::std::mem::align_of::<DerivedWithNoVirtualMethods>(),
        8usize,
        concat!("Alignment of ", stringify!(DerivedWithNoVirtualMethods))
    );
}
impl Default for DerivedWithNoVirtualMethods {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// This should not have an explicit vtable.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DerivedWithVirtualMethods {
    pub _base: BaseWithVtable<*mut ::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_DerivedWithVirtualMethods() {
    assert_eq!(
        ::std::mem::size_of::<DerivedWithVirtualMethods>(),
        16usize,
        concat!("Size of: ", stringify!(DerivedWithVirtualMethods))
    );
    assert_eq!(
        ::std::mem::align_of::<DerivedWithVirtualMethods>(),
        8usize,
        concat!("Alignment of ", stringify!(DerivedWithVirtualMethods))
    );
}
impl Default for DerivedWithVirtualMethods {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// This should not have any vtable.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BaseWithoutVtable<U> {
    pub u: U,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<U>>,
}
impl<U> Default for BaseWithoutVtable<U> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct DerivedWithVtable__bindgen_vtable(::std::os::raw::c_void);
/// This should have an explicit vtable.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DerivedWithVtable {
    pub vtable_: *const DerivedWithVtable__bindgen_vtable,
    pub _base: BaseWithoutVtable<*mut ::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_DerivedWithVtable() {
    assert_eq!(
        ::std::mem::size_of::<DerivedWithVtable>(),
        16usize,
        concat!("Size of: ", stringify!(DerivedWithVtable))
    );
    assert_eq!(
        ::std::mem::align_of::<DerivedWithVtable>(),
        8usize,
        concat!("Alignment of ", stringify!(DerivedWithVtable))
    );
}
impl Default for DerivedWithVtable {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// This should not have any vtable.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DerivedWithoutVtable {
    pub _base: BaseWithoutVtable<*mut ::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_DerivedWithoutVtable() {
    assert_eq!(
        ::std::mem::size_of::<DerivedWithoutVtable>(),
        8usize,
        concat!("Size of: ", stringify!(DerivedWithoutVtable))
    );
    assert_eq!(
        ::std::mem::align_of::<DerivedWithoutVtable>(),
        8usize,
        concat!("Alignment of ", stringify!(DerivedWithoutVtable))
    );
}
impl Default for DerivedWithoutVtable {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn __bindgen_test_layout_BaseWithVtable_open0_ptr_char_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<BaseWithVtable<*mut ::std::os::raw::c_char>>(),
        16usize,
        concat!(
            "Size of template specialization: ",
            stringify!(BaseWithVtable<*mut ::std::os::raw::c_char>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<BaseWithVtable<*mut ::std::os::raw::c_char>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(BaseWithVtable<*mut ::std::os::raw::c_char>)
        )
    );
}
#[test]
fn __bindgen_test_layout_BaseWithVtable_open0_ptr_char_close0_instantiation_1()
{
    assert_eq!(
        ::std::mem::size_of::<BaseWithVtable<*mut ::std::os::raw::c_char>>(),
        16usize,
        concat!(
            "Size of template specialization: ",
            stringify!(BaseWithVtable<*mut ::std::os::raw::c_char>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<BaseWithVtable<*mut ::std::os::raw::c_char>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(BaseWithVtable<*mut ::std::os::raw::c_char>)
        )
    );
}
#[test]
fn __bindgen_test_layout_BaseWithoutVtable_open0_ptr_char_close0_instantiation()
{
    assert_eq!(
        ::std::mem::size_of::<BaseWithoutVtable<*mut ::std::os::raw::c_char>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(BaseWithoutVtable<*mut ::std::os::raw::c_char>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<BaseWithoutVtable<*mut ::std::os::raw::c_char>>(
        ),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(BaseWithoutVtable<*mut ::std::os::raw::c_char>)
        )
    );
}
#[test]
fn __bindgen_test_layout_BaseWithoutVtable_open0_ptr_char_close0_instantiation_1(
) {
    assert_eq!(
        ::std::mem::size_of::<BaseWithoutVtable<*mut ::std::os::raw::c_char>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(BaseWithoutVtable<*mut ::std::os::raw::c_char>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<BaseWithoutVtable<*mut ::std::os::raw::c_char>>(
        ),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(BaseWithoutVtable<*mut ::std::os::raw::c_char>)
        )
    );
}
