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
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Packed>())).a as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Packed), "::", stringify!(a))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Packed>())).b as *const _ as usize },
        2usize,
        concat!("Offset of field: ", stringify!(Packed), "::", stringify!(b))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Packed>())).c as *const _ as usize },
        4usize,
        concat!("Offset of field: ", stringify!(Packed), "::", stringify!(c))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Packed>())).d as *const _ as usize },
        6usize,
        concat!("Offset of field: ", stringify!(Packed), "::", stringify!(d))
    );
}
