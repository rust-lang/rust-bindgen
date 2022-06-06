#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C, packed(2))]
#[derive(Debug, Default, Copy, Clone)]
pub struct Packed {
    pub a: ::std::os::raw::c_char,
    pub b: ::std::os::raw::c_short,
    pub c: ::std::os::raw::c_char,
    pub d: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Packed() {
    assert_eq!(
        ::std::mem::size_of::<Packed>(),
        10usize,
        concat!("Size of: ", stringify!(Packed))
    );
    assert_eq!(
        ::std::mem::align_of::<Packed>(),
        2usize,
        concat!("Alignment of ", stringify!(Packed))
    );
    fn test_field_a() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Packed>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Packed),
                "::",
                stringify!(a)
            )
        );
    }
    test_field_a();
    fn test_field_b() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Packed>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize
            },
            2usize,
            concat!(
                "Offset of field: ",
                stringify!(Packed),
                "::",
                stringify!(b)
            )
        );
    }
    test_field_b();
    fn test_field_c() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Packed>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).c) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(Packed),
                "::",
                stringify!(c)
            )
        );
    }
    test_field_c();
    fn test_field_d() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Packed>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).d) as usize - ptr as usize
            },
            6usize,
            concat!(
                "Offset of field: ",
                stringify!(Packed),
                "::",
                stringify!(d)
            )
        );
    }
    test_field_d();
}
