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
        {
            let struct_instance = unsafe { std::mem::zeroed::<AlignedToOne>() };
            let struct_ptr = &struct_instance as *const AlignedToOne;
            let field_ptr = std::ptr::addr_of!(struct_instance.i);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
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
        {
            let struct_instance = unsafe { std::mem::zeroed::<AlignedToTwo>() };
            let struct_ptr = &struct_instance as *const AlignedToTwo;
            let field_ptr = std::ptr::addr_of!(struct_instance.i);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
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
        {
            let struct_instance = unsafe { std::mem::zeroed::<PackedToOne>() };
            let struct_ptr = &struct_instance as *const PackedToOne;
            let field_ptr = std::ptr::addr_of!(struct_instance.x);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PackedToOne),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<PackedToOne>() };
            let struct_ptr = &struct_instance as *const PackedToOne;
            let field_ptr = std::ptr::addr_of!(struct_instance.y);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
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
        {
            let struct_instance = unsafe { std::mem::zeroed::<PackedToTwo>() };
            let struct_ptr = &struct_instance as *const PackedToTwo;
            let field_ptr = std::ptr::addr_of!(struct_instance.x);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PackedToTwo),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<PackedToTwo>() };
            let struct_ptr = &struct_instance as *const PackedToTwo;
            let field_ptr = std::ptr::addr_of!(struct_instance.y);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(PackedToTwo),
            "::",
            stringify!(y)
        )
    );
}
