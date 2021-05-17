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
            assert_eq!(
                {
                    let struct_instance =
                        unsafe { std::mem::zeroed::<Bar_Baz>() };
                    let struct_ptr = &struct_instance as *const Bar_Baz;
                    let field_ptr = std::ptr::addr_of!(struct_instance.foo);
                    let struct_address = struct_ptr as usize;
                    let field_address = field_ptr as usize;
                    std::mem::forget(struct_instance);
                    field_address.checked_sub(struct_address).unwrap()
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
            assert_eq!(
                {
                    let struct_instance = unsafe { std::mem::zeroed::<Bar>() };
                    let struct_ptr = &struct_instance as *const Bar;
                    let field_ptr = std::ptr::addr_of!(struct_instance.foo);
                    let struct_address = struct_ptr as usize;
                    let field_address = field_ptr as usize;
                    std::mem::forget(struct_instance);
                    field_address.checked_sub(struct_address).unwrap()
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
            assert_eq!(
                {
                    let struct_instance = unsafe { std::mem::zeroed::<Baz>() };
                    let struct_ptr = &struct_instance as *const Baz;
                    let field_ptr = std::ptr::addr_of!(struct_instance.baz);
                    let struct_address = struct_ptr as usize;
                    let field_address = field_ptr as usize;
                    std::mem::forget(struct_instance);
                    field_address.checked_sub(struct_address).unwrap()
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
    }
}
