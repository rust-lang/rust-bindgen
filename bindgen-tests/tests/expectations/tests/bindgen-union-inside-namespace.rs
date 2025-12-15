#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod foo {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Clone, Copy)]
        pub union Bar {
            pub foo: ::std::os::raw::c_int,
            pub bar: ::std::os::raw::c_int,
        }
        #[allow(clippy::unnecessary_operation, clippy::identity_op)]
        const _: () = {
            ["Size of Bar"][::std::mem::size_of::<Bar>() - 4usize];
            ["Alignment of Bar"][::std::mem::align_of::<Bar>() - 4usize];
            ["Offset of field: Bar::foo"][::std::mem::offset_of!(Bar, foo) - 0usize];
            ["Offset of field: Bar::bar"][::std::mem::offset_of!(Bar, bar) - 0usize];
        };
        impl Default for Bar {
            fn default() -> Self {
                let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
                unsafe {
                    ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                    s.assume_init()
                }
            }
        }
    }
}
