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
        /// <div rustbindgen replaces="foo::Bar"></div>
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Bar {
            pub bazz: ::std::os::raw::c_int,
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
                    let field_ptr = std::ptr::addr_of!(struct_instance.bazz);
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
                    stringify!(bazz)
                )
            );
        }
    }
    pub type ReferencesBar = root::foo::Bar;
}
