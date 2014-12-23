#![feature(phase)]

#[phase(plugin)]
extern crate bindgen;

extern crate libc;

#[test]
fn test_union_with_anon_struct() {
    // XXX: Rustc thinks that the anonymous struct, bar, is unused.
    #[allow(dead_code)]
    mod ffi { bindgen!("headers/union_with_anon_struct.h"); }
    let mut x = ffi::Union_foo { _bindgen_data_: [0, 0] };

    unsafe {
        (*x.bar()).a = 0x12345678;
        (*x.bar()).b = 0x87654321;

        assert_eq!((*x.bar()).a, 0x12345678);
        assert_eq!((*x.bar()).b, 0x87654321);
    }
}

#[test]
fn test_union_with_anon_union() {
    mod ffi { bindgen!("headers/union_with_anon_union.h"); }
    let mut x = ffi::Union_foo { _bindgen_data_: [0] };

    unsafe {
        *(*x.bar()).a() = 0x12345678;

        assert_eq!(*(*x.bar()).a(), 0x12345678);
        assert_eq!(*(*x.bar()).b(), 0x5678);
    }
}

#[test]
fn test_union_with_anon_unnamed_struct() {
    mod ffi { bindgen!("headers/union_with_anon_unnamed_struct.h"); }
    let mut x = ffi::Union_pixel { _bindgen_data_: [0] };

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
fn test_union_with_anon_unnamed_union() {
    mod ffi { bindgen!("headers/union_with_anon_unnamed_union.h"); }
    let mut x = ffi::Union_foo { _bindgen_data_: [0] };

    unsafe {
        *x.a() = 0x12345678;

        assert_eq!(*x.a(), 0x12345678);
        assert_eq!(*x.b(), 0x5678);
        assert_eq!(*x.c(), 0x78);
    }
}

#[test]
fn test_union_with_nesting() {
    mod ffi { bindgen!("headers/union_with_nesting.h"); }
    let mut x = ffi::Union_foo { _bindgen_data_: [0] };

    unsafe {
        *x.a() = 0x12345678;

        assert_eq!(*x.a(), 0x12345678);
        assert_eq!(*x.b1(), 0x5678);
        assert_eq!(*x.b2(), 0x5678);
        assert_eq!(*x.c1(), 0x1234);
        assert_eq!(*x.c2(), 0x1234);
    }
}
