#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub const __: ::std::os::raw::c_int = 10;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ptr_t {
    pub __: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout_ptr_t() {
    assert_eq!(
        ::std::mem::size_of::<ptr_t>(),
        8usize,
        concat!("Size of: ", stringify!(ptr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ptr_t>(),
        1usize,
        concat!("Alignment of ", stringify!(ptr_t))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<ptr_t>() };
            let struct_ptr = &struct_instance as *const ptr_t;
            let field_ptr = std::ptr::addr_of!(struct_instance.__);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(ptr_t), "::", stringify!(__))
    );
}
