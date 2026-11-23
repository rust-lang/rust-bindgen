#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C" {
    #[link_name = "\u{1}func_a_redir"]
    pub fn func_a();
}
unsafe extern "C" {
    #[link_name = "\u{1}func_b_redir"]
    pub fn func_b();
}
unsafe extern "C" {
    #[link_name = "\u{1}func_c_redir"]
    pub fn func_c();
}
unsafe extern "C" {
    #[link_name = "\u{1}func_d_redir"]
    pub fn func_d();
}
