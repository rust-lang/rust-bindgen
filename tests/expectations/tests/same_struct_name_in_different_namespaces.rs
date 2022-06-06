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
    fn test_field_x() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<JS_shadow_Zone>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(JS_shadow_Zone),
                "::",
                stringify!(x)
            )
        );
    }
    test_field_x();
    fn test_field_y() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<JS_shadow_Zone>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(JS_shadow_Zone),
                "::",
                stringify!(y)
            )
        );
    }
    test_field_y();
}
