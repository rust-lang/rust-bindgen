#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct nsRefPtrHashtable {
        pub _address: u8,
    }
    pub type nsRefPtrHashtable_UserDataType<PtrType> = *mut PtrType;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct a {
        pub b: u8,
    }
    #[test]
    fn bindgen_test_layout_a() {
        assert_eq!(
            ::std::mem::size_of::<a>(),
            1usize,
            concat!("Size of: ", stringify!(a))
        );
        assert_eq!(
            ::std::mem::align_of::<a>(),
            1usize,
            concat!("Alignment of ", stringify!(a))
        );
        fn test_field_b() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<a>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(a),
                    "::",
                    stringify!(b)
                )
            );
        }
        test_field_b();
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct nsCSSValue {
        pub c: root::a,
    }
    #[test]
    fn bindgen_test_layout_nsCSSValue() {
        assert_eq!(
            ::std::mem::size_of::<nsCSSValue>(),
            1usize,
            concat!("Size of: ", stringify!(nsCSSValue))
        );
        assert_eq!(
            ::std::mem::align_of::<nsCSSValue>(),
            1usize,
            concat!("Alignment of ", stringify!(nsCSSValue))
        );
        fn test_field_c() {
            assert_eq!(
                unsafe {
                    let uninit =
                        ::std::mem::MaybeUninit::<nsCSSValue>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).c) as usize - ptr as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(nsCSSValue),
                    "::",
                    stringify!(c)
                )
            );
        }
        test_field_c();
    }
}
