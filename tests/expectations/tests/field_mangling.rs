#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct foo {
    pub type_: ::std::os::raw::c_int,
    pub type_: ::std::os::raw::c_int,
    pub type__: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_foo() {
    const UNINIT: ::std::mem::MaybeUninit<foo> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        12usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        4usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(foo),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(foo),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type__) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(foo),
            "::",
            stringify!(type__)
        )
    );
}
