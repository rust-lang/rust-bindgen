#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union Bar {
    pub foo: ::std::os::raw::c_uchar,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(
        ::std::mem::size_of::<Bar>(),
        16usize,
        concat!("Size of: ", stringify!(Bar))
    );
    assert_eq!(
        ::std::mem::align_of::<Bar>(),
        16usize,
        concat!("Alignment of ", stringify!(Bar))
    );
    fn test_field_foo() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Bar>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).foo) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Bar),
                "::",
                stringify!(foo)
            )
        );
    }
    test_field_foo();
}
impl Default for Bar {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union Baz {
    pub bar: Bar,
}
#[test]
fn bindgen_test_layout_Baz() {
    assert_eq!(
        ::std::mem::size_of::<Baz>(),
        16usize,
        concat!("Size of: ", stringify!(Baz))
    );
    assert_eq!(
        ::std::mem::align_of::<Baz>(),
        16usize,
        concat!("Alignment of ", stringify!(Baz))
    );
    fn test_field_bar() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Baz>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Baz),
                "::",
                stringify!(bar)
            )
        );
    }
    test_field_bar();
}
impl Default for Baz {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
