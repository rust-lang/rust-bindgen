#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type FuncType = ::std::option::Option<unsafe extern "C" fn()>;
extern "C" {
    pub fn Func();
}
