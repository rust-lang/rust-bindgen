#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C, packed(2))]
#[derive(Debug, Default, Copy, Clone)]
pub struct Test {
    pub x: ::std::os::raw::c_ulong,
    pub a: ::std::os::raw::c_char,
    pub b: ::std::os::raw::c_char,
    pub c: ::std::os::raw::c_char,
    pub __bindgen_padding_0: u8,
}
#[test]
fn bindgen_test_layout_Test() {
    const UNINIT: ::std::mem::MaybeUninit<Test> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Test>(),
        12usize,
        concat!("Size of: ", stringify!(Test)),
    );
    assert_eq!(
        ::std::mem::align_of::<Test>(),
        2usize,
        concat!("Alignment of ", stringify!(Test)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(x)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        8usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(a)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        9usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(b)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).c) as usize - ptr as usize },
        10usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(c)),
    );
}
