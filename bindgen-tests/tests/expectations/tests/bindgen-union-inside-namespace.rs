#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[repr(C)]
    pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
    impl<T> __BindgenUnionField<T> {
        #[inline]
        pub fn new() -> Self {
            __BindgenUnionField(::std::marker::PhantomData)
        }
        #[inline]
        pub unsafe fn as_ref(&self) -> &T {
            ::std::mem::transmute(self)
        }
        #[inline]
        pub unsafe fn as_mut(&mut self) -> &mut T {
            ::std::mem::transmute(self)
        }
    }
    impl<T> ::std::default::Default for __BindgenUnionField<T> {
        #[inline]
        fn default() -> Self {
            Self::new()
        }
    }
    impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }
    impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
    impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            fmt.write_str("__BindgenUnionField")
        }
    }
    impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
        fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
    }
    impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
        fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
            true
        }
    }
    impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
    #[allow(unused_imports)]
    use self::super::root;
    pub mod foo {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Default, Copy)]
        pub struct Bar {
            pub foo: root::__BindgenUnionField<::std::os::raw::c_int>,
            pub bar: root::__BindgenUnionField<::std::os::raw::c_int>,
            pub bindgen_union_field: u32,
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
        impl Clone for Bar {
            fn clone(&self) -> Self {
                *self
            }
        }
    }
}
