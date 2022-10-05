#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug)]
pub struct extern_type;

#[repr(C)]
#[derive(Debug)]
pub struct local_type {
    pub inner: extern_type,
}
#[test]
fn bindgen_test_layout_local_type() {
    const UNINIT: ::std::mem::MaybeUninit<local_type> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<local_type>(),
        0usize,
        concat!("Size of: ", stringify!(local_type))
    );
    assert_eq!(
        ::std::mem::align_of::<local_type>(),
        1usize,
        concat!("Alignment of ", stringify!(local_type))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inner) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(local_type),
            "::",
            stringify!(inner)
        )
    );
}
