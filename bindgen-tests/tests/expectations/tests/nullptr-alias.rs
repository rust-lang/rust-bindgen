#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub type nullptr_t = *const ::std::os::raw::c_void;
    pub mod nested {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub type nullptr_t = *const ::std::os::raw::c_void;
        unsafe extern "C" {
            #[link_name = "\u{1}_ZN6nested8identityEDn"]
            pub fn identity(value: root::nested::nullptr_t) -> root::nested::nullptr_t;
        }
    }
    unsafe extern "C" {
        #[link_name = "\u{1}_Z8identityDn"]
        pub fn identity(value: root::nullptr_t) -> root::nullptr_t;
    }
}
