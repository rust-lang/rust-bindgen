#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type Array16 = u8;
pub type ArrayInt4 = [u32; 4usize];
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct UsesArray {
    pub array_char_16: [u8; 16usize],
    pub array_bool_8: [u8; 8usize],
    pub array_int_4: ArrayInt4,
}
#[test]
fn bindgen_test_layout_UsesArray() {
    assert_eq!(
        ::std::mem::size_of::<UsesArray>(),
        40usize,
        concat!("Size of: ", stringify!(UsesArray))
    );
    assert_eq!(
        ::std::mem::align_of::<UsesArray>(),
        4usize,
        concat!("Alignment of ", stringify!(UsesArray))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<UsesArray>() };
            let struct_ptr = &struct_instance as *const UsesArray;
            let field_ptr = std::ptr::addr_of!(struct_instance.array_char_16);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(UsesArray),
            "::",
            stringify!(array_char_16)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<UsesArray>() };
            let struct_ptr = &struct_instance as *const UsesArray;
            let field_ptr = std::ptr::addr_of!(struct_instance.array_bool_8);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(UsesArray),
            "::",
            stringify!(array_bool_8)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<UsesArray>() };
            let struct_ptr = &struct_instance as *const UsesArray;
            let field_ptr = std::ptr::addr_of!(struct_instance.array_int_4);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(UsesArray),
            "::",
            stringify!(array_int_4)
        )
    );
}
