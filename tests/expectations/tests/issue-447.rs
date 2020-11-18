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
            struct Box_GuardObjectNotifier {
                ptr: *mut ::std::ffi::c_void,
            }
            impl Box_GuardObjectNotifier {}
            impl Drop for Box_GuardObjectNotifier {
                fn drop(&mut self) {
                    unsafe {
                        ::std::alloc::dealloc(
                            self.ptr as *mut u8,
                            ::std::alloc::Layout::from_size_align(
                                1usize, 1usize,
                            )
                            .unwrap(),
                        );
                    }
                }
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
    struct Box_JSAutoCompartment {
        ptr: *mut ::std::ffi::c_void,
    }
    impl Box_JSAutoCompartment {
        #[inline]
        pub fn new(arg1: root::mozilla::detail::GuardObjectNotifier) -> Self {
            unsafe {
                let ret = Self {
                    ptr: ::std::alloc::alloc(
                        ::std::alloc::Layout::from_size_align(1usize, 1usize)
                            .unwrap(),
                    ) as *mut ::std::ffi::c_void,
                };
                JSAutoCompartment_JSAutoCompartment(
                    ret.ptr as *mut JSAutoCompartment,
                    arg1,
                );
                ret
            }
        }
    }
    impl Drop for Box_JSAutoCompartment {
        fn drop(&mut self) {
            unsafe {
                ::std::alloc::dealloc(
                    self.ptr as *mut u8,
                    ::std::alloc::Layout::from_size_align(1usize, 1usize)
                        .unwrap(),
                );
            }
        }
    }
}
