#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct SizedIntegers {
    pub x: u8,
    pub y: u16,
    pub z: u32,
}
#[test]
fn bindgen_test_layout_SizedIntegers() {
    assert_eq!(
        ::std::mem::size_of::<SizedIntegers>(),
        8usize,
        concat!("Size of: ", stringify!(SizedIntegers))
    );
    assert_eq!(
        ::std::mem::align_of::<SizedIntegers>(),
        4usize,
        concat!("Alignment of ", stringify!(SizedIntegers))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<SizedIntegers>())).x as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SizedIntegers),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<SizedIntegers>())).y as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(SizedIntegers),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<SizedIntegers>())).z as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(SizedIntegers),
            "::",
            stringify!(z)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct StructWithBlocklistedFwdDecl {
    pub b: u8,
}
#[test]
fn bindgen_test_layout_StructWithBlocklistedFwdDecl() {
    assert_eq!(
        ::std::mem::size_of::<StructWithBlocklistedFwdDecl>(),
        1usize,
        concat!("Size of: ", stringify!(StructWithBlocklistedFwdDecl))
    );
    assert_eq!(
        ::std::mem::align_of::<StructWithBlocklistedFwdDecl>(),
        1usize,
        concat!("Alignment of ", stringify!(StructWithBlocklistedFwdDecl))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<StructWithBlocklistedFwdDecl>())).b
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(StructWithBlocklistedFwdDecl),
            "::",
            stringify!(b)
        )
    );
}
