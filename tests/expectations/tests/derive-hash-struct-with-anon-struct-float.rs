#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// A struct containing a struct containing a float that cannot derive Hash/Eq/Ord but can derive PartialEq/PartialOrd
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct foo {
    pub bar: foo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct foo__bindgen_ty_1 {
    pub a: f32,
    pub b: f32,
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
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        8usize,
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
}
