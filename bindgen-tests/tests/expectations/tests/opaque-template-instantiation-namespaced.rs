#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod zoidberg {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct Template<T> {
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
            pub member: T,
        }
        impl<T> Default for Template<T> {
            fn default() -> Self {
                let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
                unsafe {
                    ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                    s.assume_init()
                }
            }
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct Foo {
            pub c: ::std::os::raw::c_char,
        }
        const _: () = {
            ["Size of Foo"][::std::mem::size_of::<Foo>() - 1usize];
            ["Alignment of Foo"][::std::mem::align_of::<Foo>() - 1usize];
            ["Offset of field: Foo::c"][::std::mem::offset_of!(Foo, c) - 0usize];
        };
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct Bar {
            pub i: ::std::os::raw::c_int,
        }
        const _: () = {
            ["Size of Bar"][::std::mem::size_of::<Bar>() - 4usize];
            ["Alignment of Bar"][::std::mem::align_of::<Bar>() - 4usize];
            ["Offset of field: Bar::i"][::std::mem::offset_of!(Bar, i) - 0usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct ContainsInstantiation {
            pub not_opaque: root::zoidberg::Template<root::zoidberg::Foo>,
        }
        const _: () = {
            [
                "Size of ContainsInstantiation",
            ][::std::mem::size_of::<ContainsInstantiation>() - 1usize];
            [
                "Alignment of ContainsInstantiation",
            ][::std::mem::align_of::<ContainsInstantiation>() - 1usize];
            [
                "Offset of field: ContainsInstantiation::not_opaque",
            ][::std::mem::offset_of!(ContainsInstantiation, not_opaque) - 0usize];
        };
        impl Default for ContainsInstantiation {
            fn default() -> Self {
                let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
                unsafe {
                    ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                    s.assume_init()
                }
            }
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct ContainsOpaqueInstantiation {
            pub opaque: u32,
        }
        const _: () = {
            [
                "Size of ContainsOpaqueInstantiation",
            ][::std::mem::size_of::<ContainsOpaqueInstantiation>() - 4usize];
            [
                "Alignment of ContainsOpaqueInstantiation",
            ][::std::mem::align_of::<ContainsOpaqueInstantiation>() - 4usize];
            [
                "Offset of field: ContainsOpaqueInstantiation::opaque",
            ][::std::mem::offset_of!(ContainsOpaqueInstantiation, opaque) - 0usize];
        };
    }
    const _: () = {
        [
            "Size of template specialization: Template_open0_Foo_close0",
        ][::std::mem::size_of::<root::zoidberg::Template<root::zoidberg::Foo>>()
            - 1usize];
        [
            "Align of template specialization: Template_open0_Foo_close0",
        ][::std::mem::align_of::<root::zoidberg::Template<root::zoidberg::Foo>>()
            - 1usize];
    };
}
