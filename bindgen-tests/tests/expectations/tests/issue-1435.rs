#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod ns {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub const AB_A: AB = 0;
        pub const AB_B: AB = 1;
        pub type AB = ::std::os::raw::c_int;
    }
    pub use self::super::root::ns::AB as AB;
    unsafe extern "C" {
        #[link_name = "\u{1}_ZL2kA"]
        pub static kA: root::AB;
    }
}
