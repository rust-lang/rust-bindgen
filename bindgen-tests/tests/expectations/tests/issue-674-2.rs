#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod JS {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Rooted {
            pub _address: u8,
        }
        pub type Rooted_ElementType<T> = T;
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct c {
        pub b: u8,
    }
    const _: () = {
        assert!(::std::mem::size_of::<c>() == 1usize, "Size of c");
        assert!(::std::mem::align_of::<c>() == 1usize, "Alignment of c");
        assert!(::std::mem::offset_of!(c, b) == 0usize, "Offset of field: c::b");
    };
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct B {
        pub a: root::c,
    }
    const _: () = {
        assert!(::std::mem::size_of::<B>() == 1usize, "Size of B");
        assert!(::std::mem::align_of::<B>() == 1usize, "Alignment of B");
        assert!(::std::mem::offset_of!(B, a) == 0usize, "Offset of field: B::a");
    };
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct StaticRefPtr {
        pub _address: u8,
    }
    const _: () = {
        assert!(
            ::std::mem::size_of::<root::StaticRefPtr>() == 1usize,
            "Size of template specialization: StaticRefPtr_open0_B_close0",
        );
        assert!(
            ::std::mem::align_of::<root::StaticRefPtr>() == 1usize,
            "Align of template specialization: StaticRefPtr_open0_B_close0",
        );
    };
}
