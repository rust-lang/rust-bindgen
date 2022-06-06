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
    fn test_field_array_char_16() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<UsesArray>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).array_char_16) as usize -
                    ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(UsesArray),
                "::",
                stringify!(array_char_16)
            )
        );
    }
    test_field_array_char_16();
    fn test_field_array_bool_8() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<UsesArray>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).array_bool_8) as usize -
                    ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(UsesArray),
                "::",
                stringify!(array_bool_8)
            )
        );
    }
    test_field_array_bool_8();
    fn test_field_array_int_4() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<UsesArray>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).array_int_4) as usize - ptr as usize
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
    test_field_array_int_4();
}
