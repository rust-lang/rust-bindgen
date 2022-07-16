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
    const UNINIT: ::std::mem::MaybeUninit<ptr_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
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
        unsafe { ::std::ptr::addr_of!((*ptr).__) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(ptr_t), "::", stringify!(__))
    );
}
