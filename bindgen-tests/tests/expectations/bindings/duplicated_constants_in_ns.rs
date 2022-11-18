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
        pub const FOO: ::std::os::raw::c_int = 4;
    }
    pub mod bar {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub const FOO: ::std::os::raw::c_int = 5;
    }
}
