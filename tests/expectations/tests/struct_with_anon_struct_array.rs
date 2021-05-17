#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo {
    pub bar: [foo__bindgen_ty_1; 2usize],
    pub baz: [[[foo__bindgen_ty_2; 4usize]; 3usize]; 2usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_1 {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<foo__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(foo__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<foo__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(foo__bindgen_ty_1))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<foo__bindgen_ty_1>() };
            let struct_ptr = &struct_instance as *const foo__bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.a);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(foo__bindgen_ty_1),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<foo__bindgen_ty_1>() };
            let struct_ptr = &struct_instance as *const foo__bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.b);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(foo__bindgen_ty_1),
            "::",
            stringify!(b)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_2 {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<foo__bindgen_ty_2>(),
        8usize,
        concat!("Size of: ", stringify!(foo__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<foo__bindgen_ty_2>(),
        4usize,
        concat!("Alignment of ", stringify!(foo__bindgen_ty_2))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<foo__bindgen_ty_2>() };
            let struct_ptr = &struct_instance as *const foo__bindgen_ty_2;
            let field_ptr = std::ptr::addr_of!(struct_instance.a);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(foo__bindgen_ty_2),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<foo__bindgen_ty_2>() };
            let struct_ptr = &struct_instance as *const foo__bindgen_ty_2;
            let field_ptr = std::ptr::addr_of!(struct_instance.b);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(foo__bindgen_ty_2),
            "::",
            stringify!(b)
        )
    );
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        208usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        4usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<foo>() };
            let struct_ptr = &struct_instance as *const foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.bar);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(bar))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<foo>() };
            let struct_ptr = &struct_instance as *const foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.baz);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        16usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(baz))
    );
}
