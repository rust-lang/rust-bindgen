#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// <div rustbindgen opaque>
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct opaque {
    pub _bindgen_opaque_blob: u32,
}
#[test]
fn bindgen_test_layout_opaque() {
    assert_eq!(
        ::std::mem::size_of::<opaque>(),
        4usize,
        concat!("Size of: ", stringify!(opaque))
    );
    assert_eq!(
        ::std::mem::align_of::<opaque>(),
        4usize,
        concat!("Alignment of ", stringify!(opaque))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct container {
    pub contained: opaque,
}
#[test]
fn bindgen_test_layout_container() {
    assert_eq!(
        ::std::mem::size_of::<container>(),
        4usize,
        concat!("Size of: ", stringify!(container))
    );
    assert_eq!(
        ::std::mem::align_of::<container>(),
        4usize,
        concat!("Alignment of ", stringify!(container))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<container>() };
            let struct_ptr = &struct_instance as *const container;
            let field_ptr = std::ptr::addr_of!(struct_instance.contained);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(container),
            "::",
            stringify!(contained)
        )
    );
}
