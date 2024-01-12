#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
extern "C" {
    #[link_name = "\u{1}_Z1iPFvvE"]
    pub fn i(arg: ::std::option::Option<unsafe extern "C" fn() -> !>);
}
extern "C" {
    #[link_name = "\u{1}_Z1jPFvvE"]
    pub fn j(arg: ::std::option::Option<unsafe extern "C" fn() -> !>) -> !;
}
