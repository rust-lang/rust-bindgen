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
    pub mod whatever {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct typedef_struct {
            pub foo: ::std::os::raw::c_int,
        }
        #[test]
        fn bindgen_test_layout_typedef_struct() {
            const UNINIT: ::std::mem::MaybeUninit<typedef_struct> =
                ::std::mem::MaybeUninit::uninit();
            let ptr = UNINIT.as_ptr();
            assert_eq!(
                ::std::mem::size_of::<typedef_struct>(),
                4usize,
                concat!("Size of: ", stringify!(typedef_struct))
            );
            assert_eq!(
                ::std::mem::align_of::<typedef_struct>(),
                4usize,
                concat!("Alignment of ", stringify!(typedef_struct))
            );
            assert_eq!(
                unsafe {
                    ::std::ptr::addr_of!((*ptr).foo) as usize - ptr as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(typedef_struct),
                    "::",
                    stringify!(foo)
                )
            );
        }
        #[repr(u32)]
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub enum typedef_enum {
            BAR = 1,
        }
    }
    pub mod _bindgen_mod_id_12 {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct typedef_struct {
            pub foo: ::std::os::raw::c_int,
        }
        #[test]
        fn bindgen_test_layout_typedef_struct() {
            const UNINIT: ::std::mem::MaybeUninit<typedef_struct> =
                ::std::mem::MaybeUninit::uninit();
            let ptr = UNINIT.as_ptr();
            assert_eq!(
                ::std::mem::size_of::<typedef_struct>(),
                4usize,
                concat!("Size of: ", stringify!(typedef_struct))
            );
            assert_eq!(
                ::std::mem::align_of::<typedef_struct>(),
                4usize,
                concat!("Alignment of ", stringify!(typedef_struct))
            );
            assert_eq!(
                unsafe {
                    ::std::ptr::addr_of!((*ptr).foo) as usize - ptr as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(typedef_struct),
                    "::",
                    stringify!(foo)
                )
            );
        }
        #[repr(u32)]
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub enum typedef_enum {
            BAR = 1,
        }
    }
}
