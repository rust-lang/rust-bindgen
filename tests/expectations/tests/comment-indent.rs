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
    /// This is a multi-line doc comment.
    ///
    /// This class is really really interesting, look!
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Foo {
        pub _address: u8,
    }
    /// This nested class is also a multi-line doc comment.
    ///
    /// This class is not so interesting, but worth a bit of docs too!
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Foo_Bar {
        pub _address: u8,
    }
    #[test]
    fn bindgen_test_layout_Foo_Bar() {
        assert_eq!(
            ::std::mem::size_of::<Foo_Bar>(),
            1usize,
            concat!("Size of: ", stringify!(Foo_Bar))
        );
        assert_eq!(
            ::std::mem::align_of::<Foo_Bar>(),
            1usize,
            concat!("Alignment of ", stringify!(Foo_Bar))
        );
    }
    struct Box_Foo_Bar {
        ptr: *mut ::std::ffi::c_void,
    }
    impl Box_Foo_Bar {}
    impl Drop for Box_Foo_Bar {
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
    #[test]
    fn bindgen_test_layout_Foo() {
        assert_eq!(
            ::std::mem::size_of::<Foo>(),
            1usize,
            concat!("Size of: ", stringify!(Foo))
        );
        assert_eq!(
            ::std::mem::align_of::<Foo>(),
            1usize,
            concat!("Alignment of ", stringify!(Foo))
        );
    }
    struct Box_Foo {
        ptr: *mut ::std::ffi::c_void,
    }
    impl Box_Foo {}
    impl Drop for Box_Foo {
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
    pub mod test {
        #[allow(unused_imports)]
        use self::super::super::root;
        /// I'm in a namespace, and thus I may be on a rust module, most of the time.
        /// My documentation is pretty extensive, I guess.
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Baz {
            /// This member is plain awesome, just amazing.
            ///
            /// It also has super-extensive docs, with even a nice ascii-art diagram.
            ///
            /// +------+          +-------+
            /// | foo  |   ---->  | bar   |
            /// +------+          +-------+
            pub member: ::std::os::raw::c_int,
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
                unsafe {
                    &(*(::std::ptr::null::<Baz>())).member as *const _ as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(Baz),
                    "::",
                    stringify!(member)
                )
            );
        }
        struct Box_Baz {
            ptr: *mut ::std::ffi::c_void,
        }
        impl Box_Baz {}
        impl Drop for Box_Baz {
            fn drop(&mut self) {
                unsafe {
                    ::std::alloc::dealloc(
                        self.ptr as *mut u8,
                        ::std::alloc::Layout::from_size_align(4usize, 4usize)
                            .unwrap(),
                    );
                }
            }
        }
        /// I'm in an inline namespace, and as such I shouldn't get generated inside
        /// a rust module, except when the relevant option is specified. Also, this
        /// comment shouldn't be misaligned.
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct InInlineNS {
            pub _address: u8,
        }
        #[test]
        fn bindgen_test_layout_InInlineNS() {
            assert_eq!(
                ::std::mem::size_of::<InInlineNS>(),
                1usize,
                concat!("Size of: ", stringify!(InInlineNS))
            );
            assert_eq!(
                ::std::mem::align_of::<InInlineNS>(),
                1usize,
                concat!("Alignment of ", stringify!(InInlineNS))
            );
        }
        struct Box_InInlineNS {
            ptr: *mut ::std::ffi::c_void,
        }
        impl Box_InInlineNS {}
        impl Drop for Box_InInlineNS {
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
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Bazz {
            pub _address: u8,
        }
        #[test]
        fn bindgen_test_layout_Bazz() {
            assert_eq!(
                ::std::mem::size_of::<Bazz>(),
                1usize,
                concat!("Size of: ", stringify!(Bazz))
            );
            assert_eq!(
                ::std::mem::align_of::<Bazz>(),
                1usize,
                concat!("Alignment of ", stringify!(Bazz))
            );
        }
        struct Box_Bazz {
            ptr: *mut ::std::ffi::c_void,
        }
        impl Box_Bazz {}
        impl Drop for Box_Bazz {
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
}
