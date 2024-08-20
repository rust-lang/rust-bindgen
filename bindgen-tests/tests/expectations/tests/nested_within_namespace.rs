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
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Bar_Baz {
            pub foo: ::std::os::raw::c_int,
        }
        #[allow(clippy::unnecessary_operation, clippy::identity_op)]
        const _: () = {
            ["Size of Bar_Baz"][::std::mem::size_of::<Bar_Baz>() - 4usize];
            ["Alignment of Bar_Baz"][::std::mem::align_of::<Bar_Baz>() - 4usize];
            [
                "Offset of field: Bar_Baz::foo",
            ][::std::mem::offset_of!(Bar_Baz, foo) - 0usize];
        };
        #[allow(clippy::unnecessary_operation, clippy::identity_op)]
        const _: () = {
            ["Size of Bar"][::std::mem::size_of::<Bar>() - 4usize];
            ["Alignment of Bar"][::std::mem::align_of::<Bar>() - 4usize];
            ["Offset of field: Bar::foo"][::std::mem::offset_of!(Bar, foo) - 0usize];
        };
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Baz {
            pub baz: ::std::os::raw::c_int,
        }
        #[allow(clippy::unnecessary_operation, clippy::identity_op)]
        const _: () = {
            ["Size of Baz"][::std::mem::size_of::<Baz>() - 4usize];
            ["Alignment of Baz"][::std::mem::align_of::<Baz>() - 4usize];
            ["Offset of field: Baz::baz"][::std::mem::offset_of!(Baz, baz) - 0usize];
        };
    }
}
