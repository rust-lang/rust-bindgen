#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod ns {
        #[allow(unused_imports)]
        use self::super::super::root;
        unsafe extern "C" {
            #[link_name = "\u{1}_ZN2ns5arrayE"]
            pub static mut array: [::std::os::raw::c_int; 3usize];
        }
    }
    unsafe extern "C" {
        pub static mut c_array: [::std::os::raw::c_int; 4usize];
    }
    unsafe extern "C" {
        pub static mut defined: [::std::os::raw::c_int; 5usize];
    }
}
