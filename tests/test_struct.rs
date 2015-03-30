use std::default::Default;

#[test]
fn with_anon_struct() {
    mod ffi { bindgen!("headers/struct_with_anon_struct.h"); }
    let mut x: ffi::Struct_foo = Default::default();

    x.bar.a = 1;
    x.bar.b = 2;

    assert_eq!(x.bar.a, 1);
    assert_eq!(x.bar.b, 2);
}

#[test]
fn with_anon_struct_array() {
    assert_bind_eq!("headers/struct_with_anon_struct_array.h", cx,
        quote_item!(cx,
            #[repr(C)]
            #[derive(Copy)]
            pub struct Struct_foo {
                pub bar: [Struct_Unnamed1; 2usize],
                pub baz: [[[Struct_Unnamed2; 4usize]; 3usize]; 2usize],
            }
        ),
        quote_item!(cx,
            impl ::std::default::Default for Struct_foo {
                fn default() -> Struct_foo { unsafe { ::std::mem::zeroed() } }
            }
        ),
        quote_item!(cx,
            #[repr(C)]
            #[derive(Copy)]
            pub struct Struct_Unnamed1 {
                pub a: ::libc::c_int,
                pub b: ::libc::c_int,
            }
        ),
        quote_item!(cx,
            impl ::std::default::Default for Struct_Unnamed1 {
                fn default() -> Struct_Unnamed1 { unsafe { ::std::mem::zeroed() } }
            }
        ),
        quote_item!(cx,
            #[repr(C)]
            #[derive(Copy)]
            pub struct Struct_Unnamed2 {
                pub a: ::libc::c_int,
                pub b: ::libc::c_int,
            }
        ),
        quote_item!(cx,
            impl ::std::default::Default for Struct_Unnamed2 {
                fn default() -> Struct_Unnamed2 { unsafe { ::std::mem::zeroed() } }
            }
        )
    );
}

#[test]
fn with_anon_struct_pointer() {
    mod ffi { bindgen!("headers/struct_with_anon_struct_pointer.h"); }
    let mut x: ffi::Struct_foo = Default::default();
    let mut unnamed: ffi::Struct_Unnamed1 = Default::default();

    unsafe {
        x.bar = &mut unnamed;

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
    let mut x: ffi::Struct_foo = Default::default();

    unsafe {
        *x.bar.a() = 0x12345678;
        assert_eq!(*x.bar.a(), 0x12345678);
        assert_eq!(*x.bar.b(), 0x5678);
    }
}

#[test]
fn with_anon_unnamed_struct() {
    mod ffi { bindgen!("headers/struct_with_anon_unnamed_struct.h"); }
    let mut x: ffi::Struct_foo = Default::default();

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
    let mut x: ffi::Struct_foo = Default::default();

    unsafe {
        *x.a() = 0x12345678;
        assert_eq!(*x.a(), 0x12345678);
        assert_eq!(*x.b(), 0x5678);
    }
}

#[test]
fn with_nesting() {
    mod ffi { bindgen!("headers/struct_with_nesting.h"); }
    let mut x: ffi::Struct_foo = Default::default();

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
            #[derive(Copy)]
            pub struct Struct_a {
                pub val_a: *mut Struct_b,
            }
        ),
        quote_item!(cx,
            impl ::std::default::Default for Struct_a {
                fn default() -> Struct_a { unsafe { ::std::mem::zeroed() } }
            }
        ),
        quote_item!(cx,
            #[repr(C)]
            #[derive(Copy)]
            pub struct Struct_b {
                pub val_b: ::libc::c_int,
            }
        ),
        quote_item!(cx,
            impl ::std::default::Default for Struct_b {
                fn default() -> Struct_b { unsafe { ::std::mem::zeroed() } }
            }
        )
    );
}

#[test]
fn with_bitfields() {
    assert_bind_eq!("headers/struct_with_bitfields.h", cx,
        quote_item!(cx,
            #[repr(C)]
            #[derive(Copy)]
            pub struct Struct_bitfield {
                pub _bindgen_bitfield_1_: ::libc::c_ushort,
                pub e: ::libc::c_int,
                pub _bindgen_bitfield_2_: ::libc::c_uint,
                pub _bindgen_bitfield_3_: ::libc::c_uint,
            }
        ),
        quote_item!(cx,
            impl ::std::default::Default for Struct_bitfield {
                fn default() -> Struct_bitfield { unsafe { ::std::mem::zeroed() } }
            }
        )
    );
}

#[test]
fn with_fwd_decl_struct() {
    mod ffi { bindgen!("headers/forward_declared_struct.h"); }

    let a = ffi::Struct_a { b: 1 };
    let c = ffi::Struct_c { d: 1 };

    assert_eq!(a.b, 1);
    assert_eq!(c.d, 1);
}

#[test]
fn default_impl() {
    mod ffi { bindgen!("headers/struct_with_nesting.h"); }

    let mut x: ffi::Struct_foo = Default::default();

    unsafe {
        assert_eq!(x.a, 0);
        assert_eq!(*x.b(), 0);
        assert_eq!(*x.c1(), 0);
        assert_eq!(*x.c2(), 0);
        assert_eq!(*x.d1(), 0);
        assert_eq!(*x.d2(), 0);
        assert_eq!(*x.d3(), 0);
        assert_eq!(*x.d4(), 0);
    }
}
