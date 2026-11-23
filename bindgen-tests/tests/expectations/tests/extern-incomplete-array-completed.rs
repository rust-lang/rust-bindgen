#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C" {
    pub static mut array: [::std::os::raw::c_int; 3usize];
}
unsafe extern "C" {
    pub static mut matrix: [[::std::os::raw::c_int; 2usize]; 4usize];
}
unsafe extern "C" {
    pub static mut tentative: [::std::os::raw::c_int; 6usize];
}
unsafe extern "C" {
    pub static mut other: [::std::os::raw::c_int; 5usize];
}
unsafe extern "C" {
    pub static mut incomplete: [::std::os::raw::c_int; 0usize];
}
