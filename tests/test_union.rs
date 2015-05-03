use std::default::Default;

use support::assert_bind_eq;

#[test]
fn with_anon_struct() {
    mod ffi { bindgen!("headers/union_with_anon_struct.h"); }
    let mut x: ffi::Union_foo = Default::default();

    unsafe {
        (*x.bar()).a = 0x12345678;
        (*x.bar()).b = 0x87654321;

        assert_eq!((*x.bar()).a, 0x12345678);
        assert_eq!((*x.bar()).b, 0x87654321);
    }
}

#[test]
fn with_anon_struct_bitfield() {
    assert_bind_eq("headers/union_with_anon_struct_bitfield.h", "
        #[repr(C)]
        #[derive(Copy)]
        pub struct Union_foo {
            pub _bindgen_data_: [u32; 1usize],
        }

        impl Union_foo {
            pub unsafe fn a(&mut self) -> *mut ::libc::c_int {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(0))
            }
        }

        impl ::std::clone::Clone for Union_foo {
            fn clone(&self) -> Self { *self }
        }

        impl ::std::default::Default for Union_foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_anon_union() {
    mod ffi { bindgen!("headers/union_with_anon_union.h"); }
    let mut x: ffi::Union_foo = Default::default();

    unsafe {
        *(*x.bar()).a() = 0x12345678;

        assert_eq!(*(*x.bar()).a(), 0x12345678);
        assert_eq!(*(*x.bar()).b(), 0x5678);
    }
}

#[test]
fn with_anon_unnamed_struct() {
    mod ffi { bindgen!("headers/union_with_anon_unnamed_struct.h"); }
    let mut x: ffi::Union_pixel = Default::default();

    unsafe {
        *x.r() = 0xca;
        *x.g() = 0xcb;
        *x.b() = 0xcc;
        *x.a() = 0xcd;

        assert_eq!(*x.rgba(), 0xcdcccbca);
        assert_eq!(*x.r(), 0xca);
        assert_eq!(*x.g(), 0xcb);
        assert_eq!(*x.b(), 0xcc);
        assert_eq!(*x.a(), 0xcd);
    }
}

#[test]
fn with_anon_unnamed_union() {
    mod ffi { bindgen!("headers/union_with_anon_unnamed_union.h"); }
    let mut x: ffi::Union_foo = Default::default();

    unsafe {
        *x.a() = 0x12345678;

        assert_eq!(*x.a(), 0x12345678);
        assert_eq!(*x.b(), 0x5678);
        assert_eq!(*x.c(), 0x78);
    }
}

#[test]
fn with_nesting() {
    mod ffi { bindgen!("headers/union_with_nesting.h"); }
    let mut x: ffi::Union_foo = Default::default();

    unsafe {
        *x.a() = 0x12345678;

        assert_eq!(*x.a(), 0x12345678);
        assert_eq!(*x.b1(), 0x5678);
        assert_eq!(*x.b2(), 0x5678);
        assert_eq!(*x.c1(), 0x1234);
        assert_eq!(*x.c2(), 0x1234);
    }
}

#[test]
fn default_impl() {
    mod ffi { bindgen!("headers/Union_with_nesting.h"); }

    let mut x: ffi::Union_foo = Default::default();

    unsafe {
        assert_eq!(*x.a(), 0);
        assert_eq!(*x.b1(), 0);
        assert_eq!(*x.b2(), 0);
        assert_eq!(*x.c1(), 0);
        assert_eq!(*x.c2(), 0);
    }
}
