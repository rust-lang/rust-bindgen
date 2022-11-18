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
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct jsval_layout {
        pub __bindgen_anon_1: root::jsval_layout__bindgen_ty_1,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct jsval_layout__bindgen_ty_1 {
        pub _address: u8,
    }
    #[test]
    fn bindgen_test_layout_jsval_layout__bindgen_ty_1() {
        assert_eq!(
            ::std::mem::size_of::<jsval_layout__bindgen_ty_1>(),
            1usize,
            concat!("Size of: ", stringify!(jsval_layout__bindgen_ty_1))
        );
        assert_eq!(
            ::std::mem::align_of::<jsval_layout__bindgen_ty_1>(),
            1usize,
            concat!("Alignment of ", stringify!(jsval_layout__bindgen_ty_1))
        );
    }
    #[test]
    fn bindgen_test_layout_jsval_layout() {
        assert_eq!(
            ::std::mem::size_of::<jsval_layout>(),
            1usize,
            concat!("Size of: ", stringify!(jsval_layout))
        );
        assert_eq!(
            ::std::mem::align_of::<jsval_layout>(),
            1usize,
            concat!("Alignment of ", stringify!(jsval_layout))
        );
    }
}
