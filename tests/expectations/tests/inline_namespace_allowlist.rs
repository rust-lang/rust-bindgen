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
    pub mod std {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub type string = *const ::std::os::raw::c_char;
    }
}
