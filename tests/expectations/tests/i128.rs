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
    fn test_field_my_signed() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).my_signed) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(foo),
                "::",
                stringify!(my_signed)
            )
        );
    }
    test_field_my_signed();
    fn test_field_my_unsigned() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).my_unsigned) as usize - ptr as usize
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
    test_field_my_unsigned();
}
