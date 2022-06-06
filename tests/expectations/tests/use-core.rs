#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![cfg(not(target_os = "windows"))]
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
    fn test_field_a() {
        assert_eq!(
            unsafe {
                let uninit = ::core::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::core::ptr::addr_of!((*ptr).a) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(foo), "::", stringify!(a))
        );
    }
    test_field_a();
    fn test_field_b() {
        assert_eq!(
            unsafe {
                let uninit = ::core::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::core::ptr::addr_of!((*ptr).b) as usize - ptr as usize
            },
            4usize,
            concat!("Offset of field: ", stringify!(foo), "::", stringify!(b))
        );
    }
    test_field_b();
    fn test_field_bar() {
        assert_eq!(
            unsafe {
                let uninit = ::core::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::core::ptr::addr_of!((*ptr).bar) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(foo),
                "::",
                stringify!(bar)
            )
        );
    }
    test_field_bar();
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
    fn test_field_bar() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::core::mem::MaybeUninit::<_bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::core::ptr::addr_of!((*ptr).bar) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_bindgen_ty_1),
                "::",
                stringify!(bar)
            )
        );
    }
    test_field_bar();
    fn test_field_baz() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::core::mem::MaybeUninit::<_bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::core::ptr::addr_of!((*ptr).baz) as usize - ptr as usize
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
    test_field_baz();
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
