#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod outer {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub mod inner {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            #[repr(C)]
            #[derive(Debug, Default, Copy, Clone)]
            pub struct Helper {
                pub _address: u8,
            }
            const _: () = {
                assert!(::std::mem::size_of::<Helper>() == 1usize, "Size of Helper");
                assert!(
                    ::std::mem::align_of::<Helper>() == 1usize,
                    "Alignment of Helper",
                );
            };
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Test {
            pub helper: root::outer::inner::Helper,
        }
        const _: () = {
            assert!(::std::mem::size_of::<Test>() == 1usize, "Size of Test");
            assert!(::std::mem::align_of::<Test>() == 1usize, "Alignment of Test");
            assert!(
                ::std::mem::offset_of!(Test, helper) == 0usize,
                "Offset of field: Test::helper",
            );
        };
    }
}
