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
    assert!(
        ::std::mem::size_of::<DerivedWithNoVirtualMethods>() == 16usize,
        "Size of DerivedWithNoVirtualMethods",
    );
    assert!(
        ::std::mem::align_of::<DerivedWithNoVirtualMethods>() == 8usize,
        "Alignment of DerivedWithNoVirtualMethods",
    );
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
    assert!(
        ::std::mem::size_of::<DerivedWithVirtualMethods>() == 16usize,
        "Size of DerivedWithVirtualMethods",
    );
    assert!(
        ::std::mem::align_of::<DerivedWithVirtualMethods>() == 8usize,
        "Alignment of DerivedWithVirtualMethods",
    );
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
    assert!(
        ::std::mem::size_of::<DerivedWithVtable>() == 16usize,
        "Size of DerivedWithVtable",
    );
    assert!(
        ::std::mem::align_of::<DerivedWithVtable>() == 8usize,
        "Alignment of DerivedWithVtable",
    );
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
    assert!(
        ::std::mem::size_of::<DerivedWithoutVtable>() == 8usize,
        "Size of DerivedWithoutVtable",
    );
    assert!(
        ::std::mem::align_of::<DerivedWithoutVtable>() == 8usize,
        "Alignment of DerivedWithoutVtable",
    );
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
    assert!(
        ::std::mem::size_of::<BaseWithVtable<*mut ::std::os::raw::c_char>>() == 16usize,
        "Size of template specialization: BaseWithVtable_open0_ptr_char_close0",
    );
    assert!(
        ::std::mem::align_of::<BaseWithVtable<*mut ::std::os::raw::c_char>>() == 8usize,
        "Align of template specialization: BaseWithVtable_open0_ptr_char_close0",
    );
};
const _: () = {
    assert!(
        ::std::mem::size_of::<BaseWithVtable<*mut ::std::os::raw::c_char>>() == 16usize,
        "Size of template specialization: BaseWithVtable_open0_ptr_char_close0",
    );
    assert!(
        ::std::mem::align_of::<BaseWithVtable<*mut ::std::os::raw::c_char>>() == 8usize,
        "Align of template specialization: BaseWithVtable_open0_ptr_char_close0",
    );
};
const _: () = {
    assert!(
        ::std::mem::size_of::<BaseWithoutVtable<*mut ::std::os::raw::c_char>>()
            == 8usize,
        "Size of template specialization: BaseWithoutVtable_open0_ptr_char_close0",
    );
    assert!(
        ::std::mem::align_of::<BaseWithoutVtable<*mut ::std::os::raw::c_char>>()
            == 8usize,
        "Align of template specialization: BaseWithoutVtable_open0_ptr_char_close0",
    );
};
const _: () = {
    assert!(
        ::std::mem::size_of::<BaseWithoutVtable<*mut ::std::os::raw::c_char>>()
            == 8usize,
        "Size of template specialization: BaseWithoutVtable_open0_ptr_char_close0",
    );
    assert!(
        ::std::mem::align_of::<BaseWithoutVtable<*mut ::std::os::raw::c_char>>()
            == 8usize,
        "Align of template specialization: BaseWithoutVtable_open0_ptr_char_close0",
    );
};
