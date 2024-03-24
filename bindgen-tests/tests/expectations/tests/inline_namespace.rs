#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod foo {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub type Ty = ::std::os::raw::c_int;
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Bar {
        pub baz: root::foo::Ty,
    }
    const _: () = {
        ["Size of Bar"][::std::mem::size_of::<Bar>() - 4usize];
        ["Alignment of Bar"][::std::mem::align_of::<Bar>() - 4usize];
        ["Offset of field: Bar::baz"][::std::mem::offset_of!(Bar, baz) - 0usize];
    };
}
