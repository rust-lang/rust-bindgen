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
                {
                    let struct_instance =
                        unsafe { std::mem::zeroed::<typedef_struct>() };
                    let struct_ptr = &struct_instance as *const typedef_struct;
                    let field_ptr = std::ptr::addr_of!(struct_instance.foo);
                    let struct_address = struct_ptr as usize;
                    let field_address = field_ptr as usize;
                    std::mem::forget(struct_instance);
                    field_address.checked_sub(struct_address).unwrap()
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
        pub struct _bindgen_ty_1 {
            pub foo: ::std::os::raw::c_int,
        }
        #[test]
        fn bindgen_test_layout__bindgen_ty_1() {
            assert_eq!(
                ::std::mem::size_of::<_bindgen_ty_1>(),
                4usize,
                concat!("Size of: ", stringify!(_bindgen_ty_1))
            );
            assert_eq!(
                ::std::mem::align_of::<_bindgen_ty_1>(),
                4usize,
                concat!("Alignment of ", stringify!(_bindgen_ty_1))
            );
            assert_eq!(
                {
                    let struct_instance =
                        unsafe { std::mem::zeroed::<_bindgen_ty_1>() };
                    let struct_ptr = &struct_instance as *const _bindgen_ty_1;
                    let field_ptr = std::ptr::addr_of!(struct_instance.foo);
                    let struct_address = struct_ptr as usize;
                    let field_address = field_ptr as usize;
                    std::mem::forget(struct_instance);
                    field_address.checked_sub(struct_address).unwrap()
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(_bindgen_ty_1),
                    "::",
                    stringify!(foo)
                )
            );
        }
        pub type typedef_struct = root::_bindgen_mod_id_12::_bindgen_ty_1;
        pub const _bindgen_mod_id_12_BAR:
            root::_bindgen_mod_id_12::_bindgen_ty_2 = _bindgen_ty_2::BAR;
        #[repr(u32)]
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub enum _bindgen_ty_2 {
            BAR = 1,
        }
        pub use self::super::super::root::_bindgen_mod_id_12::_bindgen_ty_2 as typedef_enum;
    }
}
