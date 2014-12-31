#[test]
fn with_anon_struct() {
    mod ffi { bindgen!("headers/struct_with_anon_struct.h"); }
    let mut x = ffi::Struct_foo { bar: ffi::Struct_Unnamed1 { a: 0, b: 0 } };

    x.bar.a = 1;
    x.bar.b = 2;

    assert_eq!(x.bar.a, 1);
    assert_eq!(x.bar.b, 2);
}

#[test]
fn with_anon_struct_array() {
    mod ffi { bindgen!("headers/struct_with_anon_struct_array.h"); }
    let mut x = ffi::Struct_foo { bar: [
        ffi::Struct_Unnamed1 { a: 0, b: 0 },
        ffi::Struct_Unnamed1 { a: 1, b: 1 } ]
    };

    x.bar[1].a = 1;
    x.bar[1].b = 2;

    assert_eq!(x.bar[1].a, 1);
    assert_eq!(x.bar[1].b, 2);
}

#[test]
fn with_anon_struct_pointer() {
    mod ffi { bindgen!("headers/struct_with_anon_struct_pointer.h"); }
    let mut unnamed = box ffi::Struct_Unnamed1 { a: 0, b: 0 };

    unsafe {
        let mut x = ffi::Struct_foo { bar: ::std::mem::transmute(unnamed) };

        (*x.bar).a = 1;
        (*x.bar).b = 2;

        assert_eq!((*x.bar).a, 1);
        assert_eq!((*x.bar).b, 2);

        ::std::ptr::read(x.bar);
    }
}

#[test]
fn with_anon_union() {
    mod ffi { bindgen!("headers/struct_with_anon_union.h"); }
    let mut x = ffi::Struct_foo { bar: ffi::Union_Unnamed1 { _bindgen_data_: [0] } };

    unsafe {
        *x.bar.a() = 0x12345678;
        assert_eq!(*x.bar.a(), 0x12345678);
        assert_eq!(*x.bar.b(), 0x5678);
    }
}

#[test]
fn with_anon_unnamed_struct() {
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
fn with_anon_unnamed_union() {
    mod ffi { bindgen!("headers/struct_with_anon_unnamed_union.h"); }
    let mut x = ffi::Struct_foo { _bindgen_data_1_: [0] };

    unsafe {
        *x.a() = 0x12345678;
        assert_eq!(*x.a(), 0x12345678);
        assert_eq!(*x.b(), 0x5678);
    }
}

#[test]
fn with_nesting() {
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

#[test]
fn containing_fwd_decl_struct() {
    assert_bind_eq!("headers/struct_containing_forward_declared_struct.h", cx,
        quote_item!(cx,
            #[repr(C)]
            #[deriving(Copy)]
            pub struct Struct_a {
                pub val_a: *mut Struct_b,
            }
        ),
        quote_item!(cx,
            #[repr(C)]
            #[deriving(Copy)]
            pub struct Struct_b {
                pub val_b: ::libc::c_int,
            }
        ));
}

#[test]
fn with_unnamed_bitfields() {
    assert_bind_eq!("headers/unnamed_bitfields.h", cx,
        quote_item!(cx,
            #[repr(C)]
            #[deriving(Copy)]
            pub struct Struct_bitfield {
                pub a: ::libc::c_ushort,
                pub b: ::libc::c_ushort,
                pub c: ::libc::c_ushort,
                pub unnamed_field1: ::libc::c_ushort,
                pub unnamed_field2: ::libc::c_ushort,
                pub d: ::libc::c_ushort,
            }
        ));
}

#[test]
fn with_fwd_decl_struct() {
    mod ffi { bindgen!("headers/forward_declared_struct.h"); }

    let a = ffi::Struct_a { b: 1 };
    let c = ffi::Struct_c { d: 1 };

    a.b;
    c.d;
}
