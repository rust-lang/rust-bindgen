#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern "C" {
    #[link_name = "\u{1}_Z1fv"]
    pub fn f() -> !;
}
extern "C" {
    #[link_name = "\u{1}_Z1gv"]
    pub fn g() -> !;
}
extern "C" {
    #[link_name = "\u{1}_Z1hv"]
    pub fn h() -> !;
}
