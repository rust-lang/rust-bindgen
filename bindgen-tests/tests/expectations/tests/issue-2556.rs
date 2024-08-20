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
    #[allow(clippy::unnecessary_operation, clippy::identity_op)]
    const _: () = {
        ["Size of nsSize"][::std::mem::size_of::<nsSize>() - 8usize];
        ["Alignment of nsSize"][::std::mem::align_of::<nsSize>() - 4usize];
        [
            "Offset of field: nsSize::width",
        ][::std::mem::offset_of!(nsSize, width) - 0usize];
        [
            "Offset of field: nsSize::height",
        ][::std::mem::offset_of!(nsSize, height) - 4usize];
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
