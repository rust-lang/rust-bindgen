#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod foo {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Bar {
            pub foo: ::std::os::raw::c_int,
            pub baz: bool,
        }
        const _: () = {
            assert!(::std::mem::size_of::<Bar>() == 8usize, "Size of Bar");
            assert!(::std::mem::align_of::<Bar>() == 4usize, "Alignment of Bar");
            assert!(
                ::std::mem::offset_of!(Bar, foo) == 0usize,
                "Offset of field: Bar::foo",
            );
            assert!(
                ::std::mem::offset_of!(Bar, baz) == 4usize,
                "Offset of field: Bar::baz",
            );
        };
    }
    pub mod bar {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct Foo {
            pub ptr: *mut root::foo::Bar,
        }
        const _: () = {
            assert!(::std::mem::size_of::<Foo>() == 8usize, "Size of Foo");
            assert!(::std::mem::align_of::<Foo>() == 8usize, "Alignment of Foo");
            assert!(
                ::std::mem::offset_of!(Foo, ptr) == 0usize,
                "Offset of field: Foo::ptr",
            );
        };
        impl Default for Foo {
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
