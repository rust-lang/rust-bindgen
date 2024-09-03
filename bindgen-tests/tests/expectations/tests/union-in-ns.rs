#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union bar {
        pub baz: ::std::os::raw::c_int,
    }
    #[allow(clippy::unnecessary_operation, clippy::identity_op)]
    const _: () = {
        ["Size of bar"][::std::mem::size_of::<bar>() - 4usize];
        ["Alignment of bar"][::std::mem::align_of::<bar>() - 4usize];
        ["Offset of field: bar::baz"][::std::mem::offset_of!(bar, baz) - 0usize];
    };
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
