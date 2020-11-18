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
    pub mod Halide {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Type {
            pub _address: u8,
        }
        extern "C" {
            #[link_name = "\u{1}_ZN6Halide4Type1bE"]
            pub static mut Type_b: root::a;
        }
        #[test]
        fn bindgen_test_layout_Type() {
            assert_eq!(
                ::std::mem::size_of::<Type>(),
                1usize,
                concat!("Size of: ", stringify!(Type))
            );
            assert_eq!(
                ::std::mem::align_of::<Type>(),
                1usize,
                concat!("Alignment of ", stringify!(Type))
            );
        }
        struct Box_Type {
            ptr: *mut ::std::ffi::c_void,
        }
        impl Box_Type {}
        impl Drop for Box_Type {
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
    #[repr(u32)]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    pub enum a {
        __bindgen_cannot_repr_c_on_empty_enum = 0,
    }
}
