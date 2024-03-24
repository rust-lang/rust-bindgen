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
        ["Size of c"][::std::mem::size_of::<c>() - 1usize];
        ["Alignment of c"][::std::mem::align_of::<c>() - 1usize];
        ["Offset of field: c::b"][::std::mem::offset_of!(c, b) - 0usize];
    };
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct B {
        pub a: root::c,
    }
    const _: () = {
        ["Size of B"][::std::mem::size_of::<B>() - 1usize];
        ["Alignment of B"][::std::mem::align_of::<B>() - 1usize];
        ["Offset of field: B::a"][::std::mem::offset_of!(B, a) - 0usize];
    };
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct StaticRefPtr {
        pub _address: u8,
    }
    const _: () = {
        [
            "Size of template specialization: StaticRefPtr_open0_B_close0",
        ][::std::mem::size_of::<root::StaticRefPtr>() - 1usize];
        [
            "Align of template specialization: StaticRefPtr_open0_B_close0",
        ][::std::mem::align_of::<root::StaticRefPtr>() - 1usize];
    };
}
