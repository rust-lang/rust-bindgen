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
        assert_eq!(
            {
                const STRUCT_SIZE: usize = std::mem::size_of::<a>();
                let buffer = [0u8; STRUCT_SIZE];
                let struct_instance = unsafe {
                    std::mem::transmute::<[u8; STRUCT_SIZE], a>(buffer)
                };
                let struct_ptr = &struct_instance as *const a;
                let field_ptr = std::ptr::addr_of!(struct_instance.b);
                let struct_address = struct_ptr as usize;
                let field_address = field_ptr as usize;
                std::mem::forget(struct_instance);
                field_address.checked_sub(struct_address).unwrap()
            },
            0usize,
            concat!("Offset of field: ", stringify!(a), "::", stringify!(b))
        );
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
        assert_eq!(
            {
                const STRUCT_SIZE: usize = std::mem::size_of::<nsCSSValue>();
                let buffer = [0u8; STRUCT_SIZE];
                let struct_instance = unsafe {
                    std::mem::transmute::<[u8; STRUCT_SIZE], nsCSSValue>(buffer)
                };
                let struct_ptr = &struct_instance as *const nsCSSValue;
                let field_ptr = std::ptr::addr_of!(struct_instance.c);
                let struct_address = struct_ptr as usize;
                let field_address = field_ptr as usize;
                std::mem::forget(struct_instance);
                field_address.checked_sub(struct_address).unwrap()
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
}
