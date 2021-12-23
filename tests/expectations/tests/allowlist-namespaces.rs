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
            #[test]
            fn bindgen_test_layout_Helper() {
                assert_eq!(
                    ::std::mem::size_of::<Helper>(),
                    1usize,
                    concat!("Size of: ", stringify!(Helper))
                );
                assert_eq!(
                    ::std::mem::align_of::<Helper>(),
                    1usize,
                    concat!("Alignment of ", stringify!(Helper))
                );
            }
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Test {
            pub helper: root::outer::inner::Helper,
        }
        #[test]
        fn bindgen_test_layout_Test() {
            assert_eq!(
                ::std::mem::size_of::<Test>(),
                1usize,
                concat!("Size of: ", stringify!(Test))
            );
            assert_eq!(
                ::std::mem::align_of::<Test>(),
                1usize,
                concat!("Alignment of ", stringify!(Test))
            );
            assert_eq!(
                unsafe {
                    &(*(::std::ptr::null::<Test>())).helper as *const _ as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(Test),
                    "::",
                    stringify!(helper)
                )
            );
        }
    }
}
