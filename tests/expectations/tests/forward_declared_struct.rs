#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct a {
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_a() {
    assert_eq!(
        ::std::mem::size_of::<a>(),
        4usize,
        concat!("Size of: ", stringify!(a))
    );
    assert_eq!(
        ::std::mem::align_of::<a>(),
        4usize,
        concat!("Alignment of ", stringify!(a))
    );
    fn test_field_b() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<a>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(a), "::", stringify!(b))
        );
    }
    test_field_b();
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct c {
    pub d: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_c() {
    assert_eq!(
        ::std::mem::size_of::<c>(),
        4usize,
        concat!("Size of: ", stringify!(c))
    );
    assert_eq!(
        ::std::mem::align_of::<c>(),
        4usize,
        concat!("Alignment of ", stringify!(c))
    );
    fn test_field_d() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<c>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).d) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(c), "::", stringify!(d))
        );
    }
    test_field_d();
}
