#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    unsafe extern "C" {
        #[link_name = "\u{1}_Z3foov"]
        pub fn foo() -> ::std::os::raw::c_int;
    }
    #[repr(transparent)]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct Foo(pub ::std::os::raw::c_int);
    unsafe extern "C" {
        #[link_name = "\u{1}_Z3barv"]
        pub fn bar() -> ::std::os::raw::c_int;
    }
    pub mod ns {
        #[allow(unused_imports)]
        use self::super::super::root;
        unsafe extern "C" {
            #[link_name = "\u{1}_ZN2ns3fooEv"]
            pub fn foo() -> ::std::os::raw::c_int;
        }
        #[repr(transparent)]
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct Bar(pub ::std::os::raw::c_int);
        unsafe extern "C" {
            #[link_name = "\u{1}_ZN2ns3barEv"]
            pub fn bar() -> ::std::os::raw::c_int;
        }
        impl Bar {
            pub const B0: root::ns::Bar = root::ns::Bar(1);
            pub const B1: root::ns::Bar = root::ns::Bar(4);
            pub const B2: root::ns::Bar = root::ns::Bar(3);
            pub const B3: root::ns::Bar = root::ns::Bar(-1);
        }
    }
    impl Foo {
        pub const Bar: root::Foo = root::Foo(2);
        pub const Baz: root::Foo = root::Foo(4);
        pub const Duplicated: root::Foo = root::Foo(4);
        pub const Negative: root::Foo = root::Foo(-3);
    }
}
