#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// <div rustbindgen derive="Debug"></div>
#[repr(C)]
#[derive(Default, Debug)]
pub struct my_type {
    pub a: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_my_type() {
    assert_eq!(
        ::std::mem::size_of::<my_type>(),
        4usize,
        concat!("Size of: ", stringify!(my_type))
    );
    assert_eq!(
        ::std::mem::align_of::<my_type>(),
        4usize,
        concat!("Alignment of ", stringify!(my_type))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<my_type>() };
            let struct_ptr = &struct_instance as *const my_type;
            let field_ptr = std::ptr::addr_of!(struct_instance.a);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(my_type),
            "::",
            stringify!(a)
        )
    );
}
/// <div rustbindgen derive="Debug"></div>
/// <div rustbindgen derive="Clone"></div>
#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct my_type2 {
    pub a: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_my_type2() {
    assert_eq!(
        ::std::mem::size_of::<my_type2>(),
        4usize,
        concat!("Size of: ", stringify!(my_type2))
    );
    assert_eq!(
        ::std::mem::align_of::<my_type2>(),
        4usize,
        concat!("Alignment of ", stringify!(my_type2))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<my_type2>() };
            let struct_ptr = &struct_instance as *const my_type2;
            let field_ptr = std::ptr::addr_of!(struct_instance.a);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(my_type2),
            "::",
            stringify!(a)
        )
    );
}
/// <div rustbindgen derive="Debug" derive="Clone"></div>
#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct my_type3 {
    pub a: ::std::os::raw::c_ulong,
}
#[test]
fn bindgen_test_layout_my_type3() {
    assert_eq!(
        ::std::mem::size_of::<my_type3>(),
        8usize,
        concat!("Size of: ", stringify!(my_type3))
    );
    assert_eq!(
        ::std::mem::align_of::<my_type3>(),
        8usize,
        concat!("Alignment of ", stringify!(my_type3))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<my_type3>() };
            let struct_ptr = &struct_instance as *const my_type3;
            let field_ptr = std::ptr::addr_of!(struct_instance.a);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(my_type3),
            "::",
            stringify!(a)
        )
    );
}
