#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod Halide {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Type {
            pub _address: u8,
        }
        extern "C" {
            #[link_name = "\u{1}_ZN6Halide4Type1bE"]
            pub static mut Type_b: root::a;
        }
        const _: () = {
            ["Size of Type"][::std::mem::size_of::<Type>() - 1usize];
            ["Alignment of Type"][::std::mem::align_of::<Type>() - 1usize];
        };
    }
    #[repr(u32)]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    pub enum a {
        __bindgen_cannot_repr_c_on_empty_enum = 0,
    }
}
