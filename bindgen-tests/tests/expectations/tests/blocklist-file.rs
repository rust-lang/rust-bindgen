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
    const UNINIT: ::std::mem::MaybeUninit<SizedIntegers> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
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
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SizedIntegers),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(SizedIntegers),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).z) as usize - ptr as usize },
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
    const UNINIT: ::std::mem::MaybeUninit<StructWithBlocklistedFwdDecl> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
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
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(StructWithBlocklistedFwdDecl),
            "::",
            stringify!(b)
        )
    );
}
