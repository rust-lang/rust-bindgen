#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type size_t = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub foo: size_t,
}
#[test]
fn bindgen_test_layout_A() {
    assert_eq!(
        ::std::mem::size_of::<A>(),
        8usize,
        concat!("Size of: ", stringify!(A))
    );
    assert_eq!(
        ::std::mem::align_of::<A>(),
        8usize,
        concat!("Alignment of ", stringify!(A))
    );
    fn test_field_foo() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<A>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).foo) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(A), "::", stringify!(foo))
        );
    }
    test_field_foo();
}
