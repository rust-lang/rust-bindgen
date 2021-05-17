#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C, packed(2))]
#[derive(Debug, Default, Copy, Clone)]
pub struct Packed {
    pub a: ::std::os::raw::c_char,
    pub b: ::std::os::raw::c_short,
    pub c: ::std::os::raw::c_char,
    pub d: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Packed() {
    assert_eq!(
        ::std::mem::size_of::<Packed>(),
        10usize,
        concat!("Size of: ", stringify!(Packed))
    );
    assert_eq!(
        ::std::mem::align_of::<Packed>(),
        2usize,
        concat!("Alignment of ", stringify!(Packed))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Packed>() };
            let struct_ptr = &struct_instance as *const Packed;
            let field_ptr = std::ptr::addr_of!(struct_instance.a);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(Packed), "::", stringify!(a))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Packed>() };
            let struct_ptr = &struct_instance as *const Packed;
            let field_ptr = std::ptr::addr_of!(struct_instance.b);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        2usize,
        concat!("Offset of field: ", stringify!(Packed), "::", stringify!(b))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Packed>() };
            let struct_ptr = &struct_instance as *const Packed;
            let field_ptr = std::ptr::addr_of!(struct_instance.c);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!("Offset of field: ", stringify!(Packed), "::", stringify!(c))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Packed>() };
            let struct_ptr = &struct_instance as *const Packed;
            let field_ptr = std::ptr::addr_of!(struct_instance.d);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        6usize,
        concat!("Offset of field: ", stringify!(Packed), "::", stringify!(d))
    );
}
