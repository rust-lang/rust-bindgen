#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
    const _: () = {
        assert!(::std::mem::size_of::<a>() == 1usize, "Size of a");
        assert!(::std::mem::align_of::<a>() == 1usize, "Alignment of a");
        assert!(::std::mem::offset_of!(a, b) == 0usize, "Offset of field: a::b");
    };
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct nsCSSValue {
        pub c: root::a,
    }
    const _: () = {
        assert!(::std::mem::size_of::<nsCSSValue>() == 1usize, "Size of nsCSSValue");
        assert!(
            ::std::mem::align_of::<nsCSSValue>() == 1usize,
            "Alignment of nsCSSValue",
        );
        assert!(
            ::std::mem::offset_of!(nsCSSValue, c) == 0usize,
            "Offset of field: nsCSSValue::c",
        );
    };
}
