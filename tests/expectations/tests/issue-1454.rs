#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug)]
pub struct extern_type;

#[repr(C)]
#[derive(Debug)]
pub struct local_type {
    pub inner: extern_type,
}
#[test]
fn bindgen_test_layout_local_type() {
    assert_eq!(
        ::std::mem::size_of::<local_type>(),
        0usize,
        concat!("Size of: ", stringify!(local_type))
    );
    assert_eq!(
        ::std::mem::align_of::<local_type>(),
        1usize,
        concat!("Alignment of ", stringify!(local_type))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<local_type>() };
            let struct_ptr = &struct_instance as *const local_type;
            let field_ptr = std::ptr::addr_of!(struct_instance.inner);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(local_type),
            "::",
            stringify!(inner)
        )
    );
}
