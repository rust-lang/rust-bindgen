#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JS_Zone {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct JS_shadow_Zone {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_JS_shadow_Zone() {
    assert_eq!(
        ::std::mem::size_of::<JS_shadow_Zone>(),
        8usize,
        concat!("Size of: ", stringify!(JS_shadow_Zone))
    );
    assert_eq!(
        ::std::mem::align_of::<JS_shadow_Zone>(),
        4usize,
        concat!("Alignment of ", stringify!(JS_shadow_Zone))
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<JS_shadow_Zone>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], JS_shadow_Zone>(buffer)
            };
            let struct_ptr = &struct_instance as *const JS_shadow_Zone;
            let field_ptr = std::ptr::addr_of!(struct_instance.x);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JS_shadow_Zone),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<JS_shadow_Zone>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], JS_shadow_Zone>(buffer)
            };
            let struct_ptr = &struct_instance as *const JS_shadow_Zone;
            let field_ptr = std::ptr::addr_of!(struct_instance.y);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(JS_shadow_Zone),
            "::",
            stringify!(y)
        )
    );
}
