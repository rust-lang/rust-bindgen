#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    /// If Bindgen could only determine the size and alignment of a
    /// type, it is represented like this.
    #[derive(PartialEq, Copy, Clone, Debug, Hash)]
    #[repr(C)]
    pub struct __BindgenOpaqueArray<T: Copy, const N: usize>(pub [T; N]);
    impl<T: Copy + Default, const N: usize> Default for __BindgenOpaqueArray<T, N> {
        fn default() -> Self {
            Self([<T as Default>::default(); N])
        }
    }
    #[allow(unused_imports)]
    use self::super::root;
    pub mod regression {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct C {
            pub a: root::__BindgenOpaqueArray<u8, 3usize>,
        }
        #[allow(clippy::unnecessary_operation, clippy::identity_op)]
        const _: () = {
            ["Size of C"][::std::mem::size_of::<C>() - 3usize];
            ["Alignment of C"][::std::mem::align_of::<C>() - 1usize];
            ["Offset of field: C::a"][::std::mem::offset_of!(C, a) - 0usize];
        };
    }
}
