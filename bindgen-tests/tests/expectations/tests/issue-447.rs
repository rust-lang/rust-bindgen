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
    pub mod mozilla {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub mod detail {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            #[repr(C)]
            #[derive(Debug, Default, Copy, Clone)]
            pub struct GuardObjectNotifier {
                pub _address: u8,
            }
            #[test]
            fn bindgen_test_layout_GuardObjectNotifier() {
                assert_eq!(
                    ::std::mem::size_of::<GuardObjectNotifier>(),
                    1usize,
                    concat!("Size of: ", stringify!(GuardObjectNotifier))
                );
                assert_eq!(
                    ::std::mem::align_of::<GuardObjectNotifier>(),
                    1usize,
                    concat!("Alignment of ", stringify!(GuardObjectNotifier))
                );
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct JSAutoCompartment {
        pub _address: u8,
    }
    #[test]
    fn bindgen_test_layout_JSAutoCompartment() {
        assert_eq!(
            ::std::mem::size_of::<JSAutoCompartment>(),
            1usize,
            concat!("Size of: ", stringify!(JSAutoCompartment))
        );
        assert_eq!(
            ::std::mem::align_of::<JSAutoCompartment>(),
            1usize,
            concat!("Alignment of ", stringify!(JSAutoCompartment))
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN17JSAutoCompartmentC1EN7mozilla6detail19GuardObjectNotifierE"]
        pub fn JSAutoCompartment_JSAutoCompartment(
            this: *mut root::JSAutoCompartment,
            arg1: root::mozilla::detail::GuardObjectNotifier,
        );
    }
    impl JSAutoCompartment {
        #[inline]
        pub unsafe fn new(
            arg1: root::mozilla::detail::GuardObjectNotifier,
        ) -> Self {
            let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
            JSAutoCompartment_JSAutoCompartment(
                __bindgen_tmp.as_mut_ptr(),
                arg1,
            );
            __bindgen_tmp.assume_init()
        }
    }
}
