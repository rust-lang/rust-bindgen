#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct nsSize {
        pub width: ::std::os::raw::c_int,
        pub height: ::std::os::raw::c_int,
    }
    #[test]
    fn bindgen_test_layout_nsSize() {
        const UNINIT: ::std::mem::MaybeUninit<nsSize> = ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::std::mem::size_of:: < nsSize > (), 8usize, concat!("Size of: ",
            stringify!(nsSize))
        );
        assert_eq!(
            ::std::mem::align_of:: < nsSize > (), 4usize, concat!("Alignment of ",
            stringify!(nsSize))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((* ptr).width) as usize - ptr as usize },
            0usize, concat!("Offset of field: ", stringify!(nsSize), "::",
            stringify!(width))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((* ptr).height) as usize - ptr as usize },
            4usize, concat!("Offset of field: ", stringify!(nsSize), "::",
            stringify!(height))
        );
    }
    pub mod foo {
        #[allow(unused_imports)]
        use self::super::super::root;
        extern "C" {
            #[link_name = "\u{1}_ZN3fooL22kFallbackIntrinsicSizeE"]
            pub static kFallbackIntrinsicSize: root::nsSize;
        }
    }
}
