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
            assert_eq!(
                unsafe {
                    &(*(::std::ptr::null::<Bar>())).foo as *const _ as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(Bar),
                    "::",
                    stringify!(foo)
                )
            );
            assert_eq!(
                unsafe {
                    &(*(::std::ptr::null::<Bar>())).baz as *const _ as usize
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
        struct Box_Bar {
            ptr: *mut ::std::ffi::c_void,
        }
        impl Box_Bar {}
        impl Drop for Box_Bar {
            fn drop(&mut self) {
                unsafe {
                    ::std::alloc::dealloc(
                        self.ptr as *mut u8,
                        ::std::alloc::Layout::from_size_align(8usize, 4usize)
                            .unwrap(),
                    );
                }
            }
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
            assert_eq!(
                unsafe {
                    &(*(::std::ptr::null::<Foo>())).ptr as *const _ as usize
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
        impl Default for Foo {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        struct Box_Foo {
            ptr: *mut ::std::ffi::c_void,
        }
        impl Box_Foo {}
        impl Drop for Box_Foo {
            fn drop(&mut self) {
                unsafe {
                    ::std::alloc::dealloc(
                        self.ptr as *mut u8,
                        ::std::alloc::Layout::from_size_align(8usize, 8usize)
                            .unwrap(),
                    );
                }
            }
        }
    }
}
