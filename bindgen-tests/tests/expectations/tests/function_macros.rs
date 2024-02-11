#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
extern "C" {
    #[link_name = "SIMPLE__extern"]
    pub fn SIMPLE();
}
extern "C" {
    #[link_name = "INDIRECT_SIMPLE__extern"]
    pub fn INDIRECT_SIMPLE();
}
extern "C" {
    #[link_name = "COMPLEX__extern"]
    pub fn COMPLEX(x: u32) -> f32;
}
extern "C" {
    #[link_name = "INDIRECT_COMPLEX__extern"]
    pub fn INDIRECT_COMPLEX(x: u32) -> f32;
}
extern "C" {
    #[link_name = "CONDITIONAL_COMPLEX__extern"]
    pub fn CONDITIONAL_COMPLEX(condition: bool, x: u32) -> f32;
}
extern "C" {
    pub fn simple();
}
extern "C" {
    pub fn complex(arg1: ::std::os::raw::c_int) -> f32;
}
