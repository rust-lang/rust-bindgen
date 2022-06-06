#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bar {
    pub u: foo,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct foo {
    pub foo: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        4usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        4usize,
        concat!("Alignment of ", stringify!(foo))
    );
    fn test_field_foo() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).foo) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(foo),
                "::",
                stringify!(foo)
            )
        );
    }
    test_field_foo();
}
#[test]
fn bindgen_test_layout_bar() {
    assert_eq!(
        ::std::mem::size_of::<bar>(),
        4usize,
        concat!("Size of: ", stringify!(bar))
    );
    assert_eq!(
        ::std::mem::align_of::<bar>(),
        4usize,
        concat!("Alignment of ", stringify!(bar))
    );
    fn test_field_u() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<bar>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).u) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(bar), "::", stringify!(u))
        );
    }
    test_field_u();
}
pub type bar_t = bar;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct baz {
    pub f: foo,
}
#[test]
fn bindgen_test_layout_baz() {
    assert_eq!(
        ::std::mem::size_of::<baz>(),
        4usize,
        concat!("Size of: ", stringify!(baz))
    );
    assert_eq!(
        ::std::mem::align_of::<baz>(),
        4usize,
        concat!("Alignment of ", stringify!(baz))
    );
    fn test_field_f() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<baz>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).f) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(baz), "::", stringify!(f))
        );
    }
    test_field_f();
}
