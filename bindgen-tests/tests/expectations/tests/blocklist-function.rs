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
    }
    pub mod bar {
        #[allow(unused_imports)]
        use self::super::super::root;
        extern "C" {
            #[link_name = "\u{1}_ZN3bar18NamespacedFunctionEv"]
            pub fn NamespacedFunction();
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct C {
        pub _address: u8,
    }
    #[test]
    fn bindgen_test_layout_C() {
        assert_eq!(
            ::std::mem::size_of::<C>(),
            1usize,
            concat!("Size of: ", stringify!(C))
        );
        assert_eq!(
            ::std::mem::align_of::<C>(),
            1usize,
            concat!("Alignment of ", stringify!(C))
        );
    }
}
