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
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Wrapper {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Wrapper_sentry {
            pub i_am_wrapper_sentry: ::std::os::raw::c_int,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct sentry {
            pub i_am_plain_sentry: bool,
        }
        #[test]
        fn bindgen_test_layout_sentry() {
            assert_eq!(
                ::std::mem::size_of::<sentry>(),
                1usize,
                concat!("Size of: ", stringify!(sentry))
            );
            assert_eq!(
                ::std::mem::align_of::<sentry>(),
                1usize,
                concat!("Alignment of ", stringify!(sentry))
            );
            assert_eq!(
                unsafe {
                    &(*(::std::ptr::null::<sentry>())).i_am_plain_sentry
                        as *const _ as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(sentry),
                    "::",
                    stringify!(i_am_plain_sentry)
                )
            );
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct NotTemplateWrapper {
            pub _address: u8,
        }
        #[test]
        fn bindgen_test_layout_NotTemplateWrapper() {
            assert_eq!(
                ::std::mem::size_of::<NotTemplateWrapper>(),
                1usize,
                concat!("Size of: ", stringify!(NotTemplateWrapper))
            );
            assert_eq!(
                ::std::mem::align_of::<NotTemplateWrapper>(),
                1usize,
                concat!("Alignment of ", stringify!(NotTemplateWrapper))
            );
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct NotTemplateWrapper_sentry {
            pub i_am_not_template_wrapper_sentry: ::std::os::raw::c_char,
        }
        #[test]
        fn bindgen_test_layout_NotTemplateWrapper_sentry() {
            assert_eq!(
                ::std::mem::size_of::<NotTemplateWrapper_sentry>(),
                1usize,
                concat!("Size of: ", stringify!(NotTemplateWrapper_sentry))
            );
            assert_eq!(
                ::std::mem::align_of::<NotTemplateWrapper_sentry>(),
                1usize,
                concat!("Alignment of ", stringify!(NotTemplateWrapper_sentry))
            );
            assert_eq!(
                unsafe {
                    &(*(::std::ptr::null::<NotTemplateWrapper_sentry>()))
                        .i_am_not_template_wrapper_sentry
                        as *const _ as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(NotTemplateWrapper_sentry),
                    "::",
                    stringify!(i_am_not_template_wrapper_sentry)
                )
            );
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct InlineNotTemplateWrapper {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct InlineNotTemplateWrapper_sentry {
            pub i_am_inline_not_template_wrapper_sentry: bool,
        }
        #[test]
        fn bindgen_test_layout_InlineNotTemplateWrapper_sentry() {
            assert_eq!(
                ::std::mem::size_of::<InlineNotTemplateWrapper_sentry>(),
                1usize,
                concat!(
                    "Size of: ",
                    stringify!(InlineNotTemplateWrapper_sentry)
                )
            );
            assert_eq!(
                ::std::mem::align_of::<InlineNotTemplateWrapper_sentry>(),
                1usize,
                concat!(
                    "Alignment of ",
                    stringify!(InlineNotTemplateWrapper_sentry)
                )
            );
            assert_eq!(
                unsafe {
                    &(*(::std::ptr::null::<InlineNotTemplateWrapper_sentry>()))
                        .i_am_inline_not_template_wrapper_sentry
                        as *const _ as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(InlineNotTemplateWrapper_sentry),
                    "::",
                    stringify!(i_am_inline_not_template_wrapper_sentry)
                )
            );
        }
        #[test]
        fn bindgen_test_layout_InlineNotTemplateWrapper() {
            assert_eq!(
                ::std::mem::size_of::<InlineNotTemplateWrapper>(),
                1usize,
                concat!("Size of: ", stringify!(InlineNotTemplateWrapper))
            );
            assert_eq!(
                ::std::mem::align_of::<InlineNotTemplateWrapper>(),
                1usize,
                concat!("Alignment of ", stringify!(InlineNotTemplateWrapper))
            );
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct InlineTemplateWrapper {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct InlineTemplateWrapper_sentry {
            pub i_am_inline_template_wrapper_sentry: ::std::os::raw::c_int,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct OuterDoubleWrapper {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct OuterDoubleWrapper_InnerDoubleWrapper {
            pub _address: u8,
        }
        #[test]
        fn bindgen_test_layout_OuterDoubleWrapper_InnerDoubleWrapper() {
            assert_eq!(
                ::std::mem::size_of::<OuterDoubleWrapper_InnerDoubleWrapper>(),
                1usize,
                concat!(
                    "Size of: ",
                    stringify!(OuterDoubleWrapper_InnerDoubleWrapper)
                )
            );
            assert_eq!(
                ::std::mem::align_of::<OuterDoubleWrapper_InnerDoubleWrapper>(),
                1usize,
                concat!(
                    "Alignment of ",
                    stringify!(OuterDoubleWrapper_InnerDoubleWrapper)
                )
            );
        }
        #[test]
        fn bindgen_test_layout_OuterDoubleWrapper() {
            assert_eq!(
                ::std::mem::size_of::<OuterDoubleWrapper>(),
                1usize,
                concat!("Size of: ", stringify!(OuterDoubleWrapper))
            );
            assert_eq!(
                ::std::mem::align_of::<OuterDoubleWrapper>(),
                1usize,
                concat!("Alignment of ", stringify!(OuterDoubleWrapper))
            );
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct OuterDoubleWrapper_InnerDoubleWrapper_sentry {
            pub i_am_double_wrapper_sentry: ::std::os::raw::c_int,
        }
        #[test]
        fn bindgen_test_layout_OuterDoubleWrapper_InnerDoubleWrapper_sentry() {
            assert_eq!(
                ::std::mem::size_of::<
                    OuterDoubleWrapper_InnerDoubleWrapper_sentry,
                >(),
                4usize,
                concat!(
                    "Size of: ",
                    stringify!(OuterDoubleWrapper_InnerDoubleWrapper_sentry)
                )
            );
            assert_eq!(
                ::std::mem::align_of::<
                    OuterDoubleWrapper_InnerDoubleWrapper_sentry,
                >(),
                4usize,
                concat!(
                    "Alignment of ",
                    stringify!(OuterDoubleWrapper_InnerDoubleWrapper_sentry)
                )
            );
            assert_eq!(
                unsafe {
                    &(*(::std::ptr::null::<
                        OuterDoubleWrapper_InnerDoubleWrapper_sentry,
                    >()))
                    .i_am_double_wrapper_sentry as *const _
                        as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(OuterDoubleWrapper_InnerDoubleWrapper_sentry),
                    "::",
                    stringify!(i_am_double_wrapper_sentry)
                )
            );
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct OuterDoubleInlineWrapper {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct OuterDoubleInlineWrapper_InnerDoubleInlineWrapper {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct OuterDoubleInlineWrapper_InnerDoubleInlineWrapper_sentry {
            pub i_am_double_wrapper_inline_sentry: ::std::os::raw::c_int,
        }
        #[test]
        fn bindgen_test_layout_OuterDoubleInlineWrapper_InnerDoubleInlineWrapper_sentry(
        ) {
            assert_eq ! (:: std :: mem :: size_of :: < OuterDoubleInlineWrapper_InnerDoubleInlineWrapper_sentry > () , 4usize , concat ! ("Size of: " , stringify ! (OuterDoubleInlineWrapper_InnerDoubleInlineWrapper_sentry)));
            assert_eq ! (:: std :: mem :: align_of :: < OuterDoubleInlineWrapper_InnerDoubleInlineWrapper_sentry > () , 4usize , concat ! ("Alignment of " , stringify ! (OuterDoubleInlineWrapper_InnerDoubleInlineWrapper_sentry)));
            assert_eq ! (unsafe { & (* (:: std :: ptr :: null :: < OuterDoubleInlineWrapper_InnerDoubleInlineWrapper_sentry > ())) . i_am_double_wrapper_inline_sentry as * const _ as usize } , 0usize , concat ! ("Offset of field: " , stringify ! (OuterDoubleInlineWrapper_InnerDoubleInlineWrapper_sentry) , "::" , stringify ! (i_am_double_wrapper_inline_sentry)));
        }
        #[test]
        fn bindgen_test_layout_OuterDoubleInlineWrapper_InnerDoubleInlineWrapper(
        ) {
            assert_eq!(
                ::std::mem::size_of::<
                    OuterDoubleInlineWrapper_InnerDoubleInlineWrapper,
                >(),
                1usize,
                concat!(
                    "Size of: ",
                    stringify!(
                        OuterDoubleInlineWrapper_InnerDoubleInlineWrapper
                    )
                )
            );
            assert_eq!(
                ::std::mem::align_of::<
                    OuterDoubleInlineWrapper_InnerDoubleInlineWrapper,
                >(),
                1usize,
                concat!(
                    "Alignment of ",
                    stringify!(
                        OuterDoubleInlineWrapper_InnerDoubleInlineWrapper
                    )
                )
            );
        }
        #[test]
        fn bindgen_test_layout_OuterDoubleInlineWrapper() {
            assert_eq!(
                ::std::mem::size_of::<OuterDoubleInlineWrapper>(),
                1usize,
                concat!("Size of: ", stringify!(OuterDoubleInlineWrapper))
            );
            assert_eq!(
                ::std::mem::align_of::<OuterDoubleInlineWrapper>(),
                1usize,
                concat!("Alignment of ", stringify!(OuterDoubleInlineWrapper))
            );
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct OutsideNamespaceWrapper {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct OutsideNamespaceWrapper_sentry {
        pub i_am_outside_namespace_wrapper_sentry: ::std::os::raw::c_int,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct sentry {
        pub i_am_outside_namespace_sentry: ::std::os::raw::c_int,
    }
    #[test]
    fn bindgen_test_layout_sentry() {
        assert_eq!(
            ::std::mem::size_of::<sentry>(),
            4usize,
            concat!("Size of: ", stringify!(sentry))
        );
        assert_eq!(
            ::std::mem::align_of::<sentry>(),
            4usize,
            concat!("Alignment of ", stringify!(sentry))
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<sentry>())).i_am_outside_namespace_sentry
                    as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(sentry),
                "::",
                stringify!(i_am_outside_namespace_sentry)
            )
        );
    }
}
