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
            pub baz: bool,
        }
        #[test]
        fn bindgen_test_layout_Bar() {
            assert_eq!(
                ::std::mem::size_of::<Bar>(),
                8usize,
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
            fn test_field_baz() {
                assert_eq!(
                    unsafe {
                        let uninit = ::std::mem::MaybeUninit::<Bar>::uninit();
                        let ptr = uninit.as_ptr();
                        ::std::ptr::addr_of!((*ptr).baz) as usize - ptr as usize
                    },
                    4usize,
                    concat!(
                        "Offset of field: ",
                        stringify!(Bar),
                        "::",
                        stringify!(baz)
                    )
                );
            }
            test_field_baz();
        }
    }
    pub mod bar {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct Foo {
            pub ptr: *mut root::foo::Bar,
        }
        #[test]
        fn bindgen_test_layout_Foo() {
            assert_eq!(
                ::std::mem::size_of::<Foo>(),
                8usize,
                concat!("Size of: ", stringify!(Foo))
            );
            assert_eq!(
                ::std::mem::align_of::<Foo>(),
                8usize,
                concat!("Alignment of ", stringify!(Foo))
            );
            fn test_field_ptr() {
                assert_eq!(
                    unsafe {
                        let uninit = ::std::mem::MaybeUninit::<Foo>::uninit();
                        let ptr = uninit.as_ptr();
                        ::std::ptr::addr_of!((*ptr).ptr) as usize - ptr as usize
                    },
                    0usize,
                    concat!(
                        "Offset of field: ",
                        stringify!(Foo),
                        "::",
                        stringify!(ptr)
                    )
                );
            }
            test_field_ptr();
        }
        impl Default for Foo {
            fn default() -> Self {
                let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
                unsafe {
                    ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                    s.assume_init()
                }
            }
        }
    }
}
