#![feature(phase)]

#[phase(plugin)]
extern crate bindgen;

extern crate libc;

#[test]
fn test_struct_with_anon_struct() {
    mod ffi { bindgen!("headers/struct_with_anon_struct.h"); }
    let mut x = ffi::Struct_foo { bar: ffi::Struct_Unnamed1 { a: 0, b: 0 } };

    x.bar.a = 1;
    x.bar.b = 2;

    assert_eq!(x.bar.a, 1);
    assert_eq!(x.bar.b, 2);
}

#[test]
fn test_struct_with_anon_union() {
    mod ffi { bindgen!("headers/struct_with_anon_union.h"); }
    let mut x = ffi::Struct_foo { bar: ffi::Union_Unnamed1 { _bindgen_data_: [0] } };

    unsafe {
        *x.bar.a() = 0x12345678;
        assert_eq!(*x.bar.a(), 0x12345678);
        assert_eq!(*x.bar.b(), 0x5678);
    }
}

#[test]
fn test_struct_with_anon_unnamed_struct() {
    mod ffi { bindgen!("headers/struct_with_anon_unnamed_struct.h"); }
    let mut x = ffi::Struct_foo { _bindgen_data_1_: [0, 0] };

    unsafe {
        *x.a() = 0x12345678;
        *x.b() = 0x87654321;
        assert_eq!(*x.a(), 0x12345678);
        assert_eq!(*x.b(), 0x87654321);
    }
}

#[test]
fn test_struct_with_anon_unnamed_union() {
    mod ffi { bindgen!("headers/struct_with_anon_unnamed_union.h"); }
    let mut x = ffi::Struct_foo { _bindgen_data_1_: [0] };

    unsafe {
        *x.a() = 0x12345678;
        assert_eq!(*x.a(), 0x12345678);
        assert_eq!(*x.b(), 0x5678);
    }
}

#[test]
fn test_struct_with_nesting() {
    mod ffi { bindgen!("headers/struct_with_nesting.h"); }
    let mut x = ffi::Struct_foo { a: 0, _bindgen_data_1_: [0] };

    unsafe {
        x.a = 0x12345678;
        *x.b() = 0x87654321;

        assert_eq!(x.a, 0x12345678);
        assert_eq!(*x.b(), 0x87654321);
        assert_eq!(*x.c1(), 0x4321);
        assert_eq!(*x.c2(), 0x8765);
        assert_eq!(*x.d1(), 0x21);
        assert_eq!(*x.d2(), 0x43);
        assert_eq!(*x.d3(), 0x65);
        assert_eq!(*x.d4(), 0x87);
    }
}
