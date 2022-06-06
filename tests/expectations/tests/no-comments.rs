#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub s: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(
        ::std::mem::size_of::<Foo>(),
        4usize,
        concat!("Size of: ", stringify!(Foo))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo>(),
        4usize,
        concat!("Alignment of ", stringify!(Foo))
    );
    fn test_field_s() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).s) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(Foo), "::", stringify!(s))
        );
    }
    test_field_s();
}
