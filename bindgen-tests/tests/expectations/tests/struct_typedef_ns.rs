#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod whatever {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct typedef_struct {
            pub foo: ::std::os::raw::c_int,
        }
        const _: () = {
            ["Size of typedef_struct"][::std::mem::size_of::<typedef_struct>() - 4usize];
            [
                "Alignment of typedef_struct",
            ][::std::mem::align_of::<typedef_struct>() - 4usize];
            [
                "Offset of field: typedef_struct::foo",
            ][::std::mem::offset_of!(typedef_struct, foo) - 0usize];
        };
        #[repr(u32)]
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub enum typedef_enum {
            BAR = 1,
        }
    }
    pub mod _bindgen_mod_id_12 {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct typedef_struct {
            pub foo: ::std::os::raw::c_int,
        }
        const _: () = {
            ["Size of typedef_struct"][::std::mem::size_of::<typedef_struct>() - 4usize];
            [
                "Alignment of typedef_struct",
            ][::std::mem::align_of::<typedef_struct>() - 4usize];
            [
                "Offset of field: typedef_struct::foo",
            ][::std::mem::offset_of!(typedef_struct, foo) - 0usize];
        };
        #[repr(u32)]
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub enum typedef_enum {
            BAR = 1,
        }
    }
}
