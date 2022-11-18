#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod foo {
        #[allow(unused_imports)]
        use self::super::super::root;
        impl root::foo::Bar {
            pub const Foo1: root::foo::Bar = Bar::Foo;
        }
        impl root::foo::Bar {
            pub const Foo3: root::foo::Bar = Bar::Foo2;
        }
        #[repr(u32)]
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub enum Bar {
            Foo = 0,
            Foo2 = 1,
        }
    }
}
