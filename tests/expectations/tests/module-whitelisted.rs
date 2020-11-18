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
    pub struct Test {
        pub _address: u8,
    }
    #[test]
    fn bindgen_test_layout_Test() {
        assert_eq!(
            ::std::mem::size_of::<Test>(),
            1usize,
            concat!("Size of: ", stringify!(Test))
        );
        assert_eq!(
            ::std::mem::align_of::<Test>(),
            1usize,
            concat!("Alignment of ", stringify!(Test))
        );
    }
    struct Box_Test {
        ptr: *mut ::std::ffi::c_void,
    }
    impl Box_Test {}
    impl Drop for Box_Test {
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
