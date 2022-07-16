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
    #[derive(Copy, Clone)]
    pub union bar {
        pub baz: ::std::os::raw::c_int,
    }
    #[test]
    fn bindgen_test_layout_bar() {
        const UNINIT: ::std::mem::MaybeUninit<bar> =
            ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::std::mem::size_of::<bar>(),
            4usize,
            concat!("Size of: ", stringify!(bar))
        );
        assert_eq!(
            ::std::mem::align_of::<bar>(),
            4usize,
            concat!("Alignment of ", stringify!(bar))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).baz) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(bar),
                "::",
                stringify!(baz)
            )
        );
    }
    impl Default for bar {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
}
