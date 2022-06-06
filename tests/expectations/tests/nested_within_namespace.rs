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
    pub mod foo {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Bar {
            pub foo: ::std::os::raw::c_int,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Bar_Baz {
            pub foo: ::std::os::raw::c_int,
        }
        #[test]
        fn bindgen_test_layout_Bar_Baz() {
            assert_eq!(
                ::std::mem::size_of::<Bar_Baz>(),
                4usize,
                concat!("Size of: ", stringify!(Bar_Baz))
            );
            assert_eq!(
                ::std::mem::align_of::<Bar_Baz>(),
                4usize,
                concat!("Alignment of ", stringify!(Bar_Baz))
            );
            fn test_field_foo() {
                assert_eq!(
                    unsafe {
                        let uninit =
                            ::std::mem::MaybeUninit::<Bar_Baz>::uninit();
                        let ptr = uninit.as_ptr();
                        ::std::ptr::addr_of!((*ptr).foo) as usize - ptr as usize
                    },
                    0usize,
                    concat!(
                        "Offset of field: ",
                        stringify!(Bar_Baz),
                        "::",
                        stringify!(foo)
                    )
                );
            }
            test_field_foo();
        }
        #[test]
        fn bindgen_test_layout_Bar() {
            assert_eq!(
                ::std::mem::size_of::<Bar>(),
                4usize,
                concat!("Size of: ", stringify!(Bar))
            );
            assert_eq!(
                ::std::mem::align_of::<Bar>(),
                4usize,
                concat!("Alignment of ", stringify!(Bar))
            );
            fn test_field_foo() {
                assert_eq!(
                    unsafe {
                        let uninit = ::std::mem::MaybeUninit::<Bar>::uninit();
                        let ptr = uninit.as_ptr();
                        ::std::ptr::addr_of!((*ptr).foo) as usize - ptr as usize
                    },
                    0usize,
                    concat!(
                        "Offset of field: ",
                        stringify!(Bar),
                        "::",
                        stringify!(foo)
                    )
                );
            }
            test_field_foo();
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Baz {
            pub baz: ::std::os::raw::c_int,
        }
        #[test]
        fn bindgen_test_layout_Baz() {
            assert_eq!(
                ::std::mem::size_of::<Baz>(),
                4usize,
                concat!("Size of: ", stringify!(Baz))
            );
            assert_eq!(
                ::std::mem::align_of::<Baz>(),
                4usize,
                concat!("Alignment of ", stringify!(Baz))
            );
            fn test_field_baz() {
                assert_eq!(
                    unsafe {
                        let uninit = ::std::mem::MaybeUninit::<Baz>::uninit();
                        let ptr = uninit.as_ptr();
                        ::std::ptr::addr_of!((*ptr).baz) as usize - ptr as usize
                    },
                    0usize,
                    concat!(
                        "Offset of field: ",
                        stringify!(Baz),
                        "::",
                        stringify!(baz)
                    )
                );
            }
            test_field_baz();
        }
    }
}
