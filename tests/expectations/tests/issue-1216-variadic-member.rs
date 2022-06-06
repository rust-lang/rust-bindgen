#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern "C" {
    pub fn f(a: ::std::os::raw::c_int, ...);
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub f: ::std::option::Option<
        unsafe extern "C" fn(
            p: *mut ::std::os::raw::c_void,
            obj: *mut ::std::os::raw::c_void,
            a: ::std::os::raw::c_int,
            ...
        ),
    >,
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(
        ::std::mem::size_of::<Foo>(),
        8usize,
        concat!("Size of: ", stringify!(Foo))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo>(),
        8usize,
        concat!("Alignment of ", stringify!(Foo))
    );
    fn test_field_f() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).f) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(Foo), "::", stringify!(f))
        );
    }
    test_field_f();
}
