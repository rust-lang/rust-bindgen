#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type VOID = ::std::os::raw::c_void;
pub type ALSO_VOID = VOID;
unsafe extern "C" {
    pub fn this_api_returns_nothing();
}
unsafe extern "C" {
    pub fn this_api_also_returns_nothing();
}
unsafe extern "C" {
    pub fn this_other_api_also_returns_nothing();
}
