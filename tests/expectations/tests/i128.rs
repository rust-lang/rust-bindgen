#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
pub struct foo {
    pub my_signed: i128,
    pub my_unsigned: u128,
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        32usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        16usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<foo>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], foo>(buffer)
            };
            let struct_ptr = &struct_instance as *const foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.my_signed);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(foo),
            "::",
            stringify!(my_signed)
        )
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<foo>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], foo>(buffer)
            };
            let struct_ptr = &struct_instance as *const foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.my_unsigned);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(foo),
            "::",
            stringify!(my_unsigned)
        )
    );
}
