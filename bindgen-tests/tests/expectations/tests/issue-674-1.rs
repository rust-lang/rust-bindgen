#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
    const _: () = {
        assert!(
            ::std::mem::size_of::<CapturingContentInfo>() == 1usize,
            "Size of CapturingContentInfo",
        );
        assert!(
            ::std::mem::align_of::<CapturingContentInfo>() == 1usize,
            "Alignment of CapturingContentInfo",
        );
        assert!(
            ::std::mem::offset_of!(CapturingContentInfo, a) == 0usize,
            "Offset of field: CapturingContentInfo::a",
        );
    };
}
