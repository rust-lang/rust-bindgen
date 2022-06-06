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
            fn test_field_bazz() {
                assert_eq!(
                    unsafe {
                        let uninit = ::std::mem::MaybeUninit::<Bar>::uninit();
                        let ptr = uninit.as_ptr();
                        ::std::ptr::addr_of!((*ptr).bazz) as usize -
                            ptr as usize
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
            test_field_bazz();
        }
    }
    pub type ReferencesBar = root::foo::Bar;
}
