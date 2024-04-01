#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
pub struct BaseWithVtable__bindgen_vtable {}
/// This should have an explicit vtable.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BaseWithVtable<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub vtable_: *const BaseWithVtable__bindgen_vtable,
    pub t: T,
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
const _: () = {
    [
        "Size of DerivedWithNoVirtualMethods",
    ][::std::mem::size_of::<DerivedWithNoVirtualMethods>() - 16usize];
    [
        "Alignment of DerivedWithNoVirtualMethods",
    ][::std::mem::align_of::<DerivedWithNoVirtualMethods>() - 8usize];
};
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
const _: () = {
    [
        "Size of DerivedWithVirtualMethods",
    ][::std::mem::size_of::<DerivedWithVirtualMethods>() - 16usize];
    [
        "Alignment of DerivedWithVirtualMethods",
    ][::std::mem::align_of::<DerivedWithVirtualMethods>() - 8usize];
};
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
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<U>>,
    pub u: U,
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
const _: () = {
    ["Size of DerivedWithVtable"][::std::mem::size_of::<DerivedWithVtable>() - 16usize];
    [
        "Alignment of DerivedWithVtable",
    ][::std::mem::align_of::<DerivedWithVtable>() - 8usize];
};
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
const _: () = {
    [
        "Size of DerivedWithoutVtable",
    ][::std::mem::size_of::<DerivedWithoutVtable>() - 8usize];
    [
        "Alignment of DerivedWithoutVtable",
    ][::std::mem::align_of::<DerivedWithoutVtable>() - 8usize];
};
impl Default for DerivedWithoutVtable {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
const _: () = {
    [
        "Size of template specialization: BaseWithVtable_open0_ptr_char_close0",
    ][::std::mem::size_of::<BaseWithVtable<*mut ::std::os::raw::c_char>>() - 16usize];
    [
        "Align of template specialization: BaseWithVtable_open0_ptr_char_close0",
    ][::std::mem::align_of::<BaseWithVtable<*mut ::std::os::raw::c_char>>() - 8usize];
};
const _: () = {
    [
        "Size of template specialization: BaseWithVtable_open0_ptr_char_close0",
    ][::std::mem::size_of::<BaseWithVtable<*mut ::std::os::raw::c_char>>() - 16usize];
    [
        "Align of template specialization: BaseWithVtable_open0_ptr_char_close0",
    ][::std::mem::align_of::<BaseWithVtable<*mut ::std::os::raw::c_char>>() - 8usize];
};
const _: () = {
    [
        "Size of template specialization: BaseWithoutVtable_open0_ptr_char_close0",
    ][::std::mem::size_of::<BaseWithoutVtable<*mut ::std::os::raw::c_char>>() - 8usize];
    [
        "Align of template specialization: BaseWithoutVtable_open0_ptr_char_close0",
    ][::std::mem::align_of::<BaseWithoutVtable<*mut ::std::os::raw::c_char>>() - 8usize];
};
const _: () = {
    [
        "Size of template specialization: BaseWithoutVtable_open0_ptr_char_close0",
    ][::std::mem::size_of::<BaseWithoutVtable<*mut ::std::os::raw::c_char>>() - 8usize];
    [
        "Align of template specialization: BaseWithoutVtable_open0_ptr_char_close0",
    ][::std::mem::align_of::<BaseWithoutVtable<*mut ::std::os::raw::c_char>>() - 8usize];
};
