#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C, packed(2))]
#[derive(Debug, Default, Copy, Clone)]
pub struct FndrOpaqueInfo {
    pub opaque: [::std::os::raw::c_char; 16usize],
}
#[test]
fn bindgen_test_layout_FndrOpaqueInfo() {
    const UNINIT: ::std::mem::MaybeUninit<FndrOpaqueInfo> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<FndrOpaqueInfo>(),
        16usize,
        concat!("Size of: ", stringify!(FndrOpaqueInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<FndrOpaqueInfo>(),
        2usize,
        concat!("Alignment of ", stringify!(FndrOpaqueInfo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).opaque) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FndrOpaqueInfo),
            "::",
            stringify!(opaque)
        )
    );
}
