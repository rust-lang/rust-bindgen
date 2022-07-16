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
    const UNINIT: ::std::mem::MaybeUninit<container> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
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
        unsafe {
            ::std::ptr::addr_of!((*ptr).contained) as usize - ptr as usize
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
