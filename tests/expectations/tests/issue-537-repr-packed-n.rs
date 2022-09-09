#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![cfg(feature = "nightly")]

/// This should not be opaque; we can see the attributes and can pack the
/// struct.
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AlignedToOne {
    pub i: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_AlignedToOne() {
    const UNINIT: ::std::mem::MaybeUninit<AlignedToOne> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AlignedToOne>(),
        4usize,
        concat!("Size of: ", stringify!(AlignedToOne))
    );
    assert_eq!(
        ::std::mem::align_of::<AlignedToOne>(),
        1usize,
        concat!("Alignment of ", stringify!(AlignedToOne))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).i) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlignedToOne),
            "::",
            stringify!(i)
        )
    );
}
/// This should be be packed because Rust 1.33 has `#[repr(packed(N))]`.
#[repr(C, packed(2))]
#[derive(Debug, Default, Copy, Clone)]
pub struct AlignedToTwo {
    pub i: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_AlignedToTwo() {
    const UNINIT: ::std::mem::MaybeUninit<AlignedToTwo> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AlignedToTwo>(),
        4usize,
        concat!("Size of: ", stringify!(AlignedToTwo))
    );
    assert_eq!(
        ::std::mem::align_of::<AlignedToTwo>(),
        2usize,
        concat!("Alignment of ", stringify!(AlignedToTwo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).i) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlignedToTwo),
            "::",
            stringify!(i)
        )
    );
}
/// This should not be opaque because although `libclang` doesn't give us the
/// `#pragma pack(1)`, we can detect that alignment is 1 and add
/// `#[repr(packed)]` to the struct ourselves.
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PackedToOne {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_PackedToOne() {
    const UNINIT: ::std::mem::MaybeUninit<PackedToOne> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<PackedToOne>(),
        8usize,
        concat!("Size of: ", stringify!(PackedToOne))
    );
    assert_eq!(
        ::std::mem::align_of::<PackedToOne>(),
        1usize,
        concat!("Alignment of ", stringify!(PackedToOne))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PackedToOne),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(PackedToOne),
            "::",
            stringify!(y)
        )
    );
}
/// This should be be packed because Rust 1.33 has `#[repr(packed(N))]`.
#[repr(C, packed(2))]
#[derive(Debug, Default, Copy, Clone)]
pub struct PackedToTwo {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_PackedToTwo() {
    const UNINIT: ::std::mem::MaybeUninit<PackedToTwo> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<PackedToTwo>(),
        8usize,
        concat!("Size of: ", stringify!(PackedToTwo))
    );
    assert_eq!(
        ::std::mem::align_of::<PackedToTwo>(),
        2usize,
        concat!("Alignment of ", stringify!(PackedToTwo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PackedToTwo),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(PackedToTwo),
            "::",
            stringify!(y)
        )
    );
}
