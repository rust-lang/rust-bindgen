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
    fn test_field_x() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<SizedIntegers>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(SizedIntegers),
                "::",
                stringify!(x)
            )
        );
    }
    test_field_x();
    fn test_field_y() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<SizedIntegers>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize
            },
            2usize,
            concat!(
                "Offset of field: ",
                stringify!(SizedIntegers),
                "::",
                stringify!(y)
            )
        );
    }
    test_field_y();
    fn test_field_z() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<SizedIntegers>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).z) as usize - ptr as usize
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
    test_field_z();
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
    fn test_field_b() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    StructWithBlocklistedFwdDecl,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize
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
    test_field_b();
}
