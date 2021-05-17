#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct a {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct _bindgen_ty_1 {
    pub ar: a,
}
#[test]
fn bindgen_test_layout__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_bindgen_ty_1>(),
        1usize,
        concat!("Size of: ", stringify!(_bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<_bindgen_ty_1>(),
        1usize,
        concat!("Alignment of ", stringify!(_bindgen_ty_1))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<_bindgen_ty_1>() };
            let struct_ptr = &struct_instance as *const _bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.ar);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bindgen_ty_1),
            "::",
            stringify!(ar)
        )
    );
}
extern "C" {
    pub static mut AutoIdVector: _bindgen_ty_1;
}
#[test]
fn __bindgen_test_layout_a_open0_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<a>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(a))
    );
    assert_eq!(
        ::std::mem::align_of::<a>(),
        1usize,
        concat!("Alignment of template specialization: ", stringify!(a))
    );
}
