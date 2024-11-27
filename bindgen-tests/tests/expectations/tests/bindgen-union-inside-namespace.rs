#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod foo {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub union Bar {
            pub foo: ::std::os::raw::c_int,
            pub bar: ::std::os::raw::c_int,
        }
        #[test]
        fn bindgen_test_layout_Bar() {
            const UNINIT: ::std::mem::MaybeUninit<Bar> = ::std::mem::MaybeUninit::uninit();
            let ptr = UNINIT.as_ptr();
            assert_eq!(::std::mem::size_of::<Bar>(), 4usize, "Size of Bar");
            assert_eq!(::std::mem::align_of::<Bar>(), 4usize, "Alignment of Bar");
            assert_eq!(
                unsafe { ::std::ptr::addr_of!((*ptr).foo) as usize - ptr as usize },
                0usize,
                "Offset of field: Bar::foo",
            );
            assert_eq!(
                unsafe { ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize },
                0usize,
                "Offset of field: Bar::bar",
            );
        }
        impl Default for Bar {
            fn default() -> Self {
                unsafe {
                    let mut s: Self = ::std::mem::uninitialized();
                    ::std::ptr::write_bytes(&mut s, 0, 1);
                    s
                }
            }
        }
    }
}
