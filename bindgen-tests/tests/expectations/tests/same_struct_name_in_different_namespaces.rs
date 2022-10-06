#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JS_Zone {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct JS_shadow_Zone {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_JS_shadow_Zone() {
    const UNINIT: ::std::mem::MaybeUninit<JS_shadow_Zone> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<JS_shadow_Zone>(),
        8usize,
        concat!("Size of: ", stringify!(JS_shadow_Zone))
    );
    assert_eq!(
        ::std::mem::align_of::<JS_shadow_Zone>(),
        4usize,
        concat!("Alignment of ", stringify!(JS_shadow_Zone))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JS_shadow_Zone),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(JS_shadow_Zone),
            "::",
            stringify!(y)
        )
    );
}
