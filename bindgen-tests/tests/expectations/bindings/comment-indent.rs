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
            const UNINIT: ::std::mem::MaybeUninit<Baz> =
                ::std::mem::MaybeUninit::uninit();
            let ptr = UNINIT.as_ptr();
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
                    ::std::ptr::addr_of!((*ptr).member) as usize - ptr as usize
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
    }
}
