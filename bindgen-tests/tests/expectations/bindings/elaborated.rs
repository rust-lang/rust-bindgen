#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type whatever_whatever_t = ::std::os::raw::c_int;
extern "C" {
    #[link_name = "\u{1}_Z9somethingPKi"]
    pub fn something(wat: *const whatever_whatever_t);
}
