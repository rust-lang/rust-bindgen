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
    pub mod mozilla {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Maybe {
            pub _address: u8,
        }
        pub type Maybe_ValueType<T> = T;
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct CapturingContentInfo {
        pub a: u8,
    }
    #[test]
    fn bindgen_test_layout_CapturingContentInfo() {
        assert_eq!(
            ::std::mem::size_of::<CapturingContentInfo>(),
            1usize,
            concat!("Size of: ", stringify!(CapturingContentInfo))
        );
        assert_eq!(
            ::std::mem::align_of::<CapturingContentInfo>(),
            1usize,
            concat!("Alignment of ", stringify!(CapturingContentInfo))
        );
        assert_eq!(
            {
                let struct_instance =
                    unsafe { std::mem::zeroed::<CapturingContentInfo>() };
                let struct_ptr =
                    &struct_instance as *const CapturingContentInfo;
                let field_ptr = std::ptr::addr_of!(struct_instance.a);
                let struct_address = struct_ptr as usize;
                let field_address = field_ptr as usize;
                std::mem::forget(struct_instance);
                field_address.checked_sub(struct_address).unwrap()
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(CapturingContentInfo),
                "::",
                stringify!(a)
            )
        );
    }
}
