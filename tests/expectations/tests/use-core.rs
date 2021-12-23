#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern crate core;

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
    pub bar: *mut ::core::ffi::c_void,
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::core::mem::size_of::<foo>(),
        16usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::core::mem::align_of::<foo>(),
        8usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<foo>())).a as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(a))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<foo>())).b as *const _ as usize },
        4usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(b))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<foo>())).bar as *const _ as usize },
        8usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(bar))
    );
}
impl Default for foo {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _bindgen_ty_1 {
    pub bar: ::std::os::raw::c_int,
    pub baz: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout__bindgen_ty_1() {
    assert_eq!(
        ::core::mem::size_of::<_bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(_bindgen_ty_1))
    );
    assert_eq!(
        ::core::mem::align_of::<_bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(_bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<_bindgen_ty_1>())).bar as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bindgen_ty_1),
            "::",
            stringify!(bar)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<_bindgen_ty_1>())).baz as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bindgen_ty_1),
            "::",
            stringify!(baz)
        )
    );
}
impl Default for _bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub static mut bazz: _bindgen_ty_1;
}
pub type fooFunction =
    ::core::option::Option<unsafe extern "C" fn(bar: ::std::os::raw::c_int)>;
