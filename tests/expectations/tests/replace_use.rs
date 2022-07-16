#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// <div rustbindgen replaces="nsTArray"></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct nsTArray {
    pub y: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Test {
    pub a: nsTArray,
}
#[test]
fn bindgen_test_layout_Test() {
    const UNINIT: ::std::mem::MaybeUninit<Test> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Test>(),
        4usize,
        concat!("Size of: ", stringify!(Test))
    );
    assert_eq!(
        ::std::mem::align_of::<Test>(),
        4usize,
        concat!("Alignment of ", stringify!(Test))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(a))
    );
}
#[test]
fn __bindgen_test_layout_nsTArray_open0_long_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<nsTArray>(),
        4usize,
        concat!("Size of template specialization: ", stringify!(nsTArray))
    );
    assert_eq!(
        ::std::mem::align_of::<nsTArray>(),
        4usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(nsTArray)
        )
    );
}
