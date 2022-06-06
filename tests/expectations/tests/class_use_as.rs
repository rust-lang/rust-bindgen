#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// <div rustbindgen="true" replaces="whatever"></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct whatever {
    pub replacement: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_whatever() {
    assert_eq!(
        ::std::mem::size_of::<whatever>(),
        4usize,
        concat!("Size of: ", stringify!(whatever))
    );
    assert_eq!(
        ::std::mem::align_of::<whatever>(),
        4usize,
        concat!("Alignment of ", stringify!(whatever))
    );
    fn test_field_replacement() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<whatever>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).replacement) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(whatever),
                "::",
                stringify!(replacement)
            )
        );
    }
    test_field_replacement();
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct container {
    pub c: whatever,
}
#[test]
fn bindgen_test_layout_container() {
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
    fn test_field_c() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<container>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).c) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(container),
                "::",
                stringify!(c)
            )
        );
    }
    test_field_c();
}
