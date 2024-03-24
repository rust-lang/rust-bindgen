#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod ns1 {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub mod ns2 {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            pub mod foo {
                pub type Type = ::std::os::raw::c_uint;
                pub const THIS: Type = 0;
                pub const SHOULD_BE: Type = 1;
                pub const A_CONSTANT: Type = 2;
            }
        }
        pub mod ns3 {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct bar {
                pub this_should_work: root::ns1::ns2::foo::Type,
            }
            const _: () = {
                ["Size of bar"][::std::mem::size_of::<bar>() - 4usize];
                ["Alignment of bar"][::std::mem::align_of::<bar>() - 4usize];
                [
                    "Offset of field: bar::this_should_work",
                ][::std::mem::offset_of!(bar, this_should_work) - 0usize];
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
    }
}
