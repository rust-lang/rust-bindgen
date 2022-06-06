#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![cfg(not(test))]

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct JNINativeInterface_ {
    pub GetVersion: ::std::option::Option<
        unsafe extern "stdcall" fn(
            env: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub __hack: ::std::os::raw::c_ulonglong,
}
#[test]
fn bindgen_test_layout_JNINativeInterface_() {
    assert_eq!(
        ::std::mem::size_of::<JNINativeInterface_>(),
        16usize,
        concat!("Size of: ", stringify!(JNINativeInterface_))
    );
    assert_eq!(
        ::std::mem::align_of::<JNINativeInterface_>(),
        8usize,
        concat!("Alignment of ", stringify!(JNINativeInterface_))
    );
    fn test_field_GetVersion() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<JNINativeInterface_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).GetVersion) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(JNINativeInterface_),
                "::",
                stringify!(GetVersion)
            )
        );
    }
    test_field_GetVersion();
    fn test_field___hack() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<JNINativeInterface_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__hack) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(JNINativeInterface_),
                "::",
                stringify!(__hack)
            )
        );
    }
    test_field___hack();
}
extern "stdcall" {
    pub fn bar();
}
