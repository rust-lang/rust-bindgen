#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// <div rustbindgen derive="Debug"></div>
#[repr(C)]
#[derive(Default, Debug)]
pub struct my_type {
    pub a: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_my_type() {
    assert_eq!(
        ::std::mem::size_of::<my_type>(),
        4usize,
        concat!("Size of: ", stringify!(my_type))
    );
    assert_eq!(
        ::std::mem::align_of::<my_type>(),
        4usize,
        concat!("Alignment of ", stringify!(my_type))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<my_type>())).a as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(my_type),
            "::",
            stringify!(a)
        )
    );
}
/// <div rustbindgen derive="Debug"></div>
/// <div rustbindgen derive="Clone"></div>
#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct my_type2 {
    pub a: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_my_type2() {
    assert_eq!(
        ::std::mem::size_of::<my_type2>(),
        4usize,
        concat!("Size of: ", stringify!(my_type2))
    );
    assert_eq!(
        ::std::mem::align_of::<my_type2>(),
        4usize,
        concat!("Alignment of ", stringify!(my_type2))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<my_type2>())).a as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(my_type2),
            "::",
            stringify!(a)
        )
    );
}
/// <div rustbindgen derive="Debug" derive="Clone"></div>
#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct my_type3 {
    pub a: ::std::os::raw::c_ulong,
}
#[test]
fn bindgen_test_layout_my_type3() {
    assert_eq!(
        ::std::mem::size_of::<my_type3>(),
        8usize,
        concat!("Size of: ", stringify!(my_type3))
    );
    assert_eq!(
        ::std::mem::align_of::<my_type3>(),
        8usize,
        concat!("Alignment of ", stringify!(my_type3))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<my_type3>())).a as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(my_type3),
            "::",
            stringify!(a)
        )
    );
}
