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
    const UNINIT: ::std::mem::MaybeUninit<foo> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
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
        unsafe {
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
    assert_eq!(
        unsafe {
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
