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
    const _: () = {
        assert!(::std::mem::size_of::<nsSize>() == 8usize, "Size of nsSize");
        assert!(::std::mem::align_of::<nsSize>() == 4usize, "Alignment of nsSize");
        assert!(
            ::std::mem::offset_of!(nsSize, width) == 0usize,
            "Offset of field: nsSize::width",
        );
        assert!(
            ::std::mem::offset_of!(nsSize, height) == 4usize,
            "Offset of field: nsSize::height",
        );
    };
    pub mod foo {
        #[allow(unused_imports)]
        use self::super::super::root;
        extern "C" {
            #[link_name = "\u{1}_ZN3fooL22kFallbackIntrinsicSizeE"]
            pub static kFallbackIntrinsicSize: root::nsSize;
        }
    }
}
