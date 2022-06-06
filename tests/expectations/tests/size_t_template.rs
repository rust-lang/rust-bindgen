#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct C {
    pub arr: [u32; 3usize],
}
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(
        ::std::mem::size_of::<C>(),
        12usize,
        concat!("Size of: ", stringify!(C))
    );
    assert_eq!(
        ::std::mem::align_of::<C>(),
        4usize,
        concat!("Alignment of ", stringify!(C))
    );
    fn test_field_arr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<C>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).arr) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(C), "::", stringify!(arr))
        );
    }
    test_field_arr();
}
