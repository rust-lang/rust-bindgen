#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    unsafe extern "C" {
        #[link_name = "\u{1}_Z4takei"]
        pub fn take(value: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    }
    pub mod _bindgen_mod_id_7 {
        #[allow(unused_imports)]
        use self::super::super::root;
    }
    pub mod Named {
        #[allow(unused_imports)]
        use self::super::super::root;
        unsafe extern "C" {
            #[link_name = "\u{1}_ZN5Named5namedEi"]
            pub fn named(value: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
        }
    }
}
