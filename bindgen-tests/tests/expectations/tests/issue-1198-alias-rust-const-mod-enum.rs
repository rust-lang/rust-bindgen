#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub mod MyDupeEnum {
    pub type Type = ::std::os::raw::c_uint;
    pub const A: Type = 0;
    pub const A_alias: Type = 0;
    pub const B: Type = 1;
}
pub mod MyOtherDupeEnum {
    pub type Type = ::std::os::raw::c_uint;
    pub const C: Type = 0;
    pub const C_alias: Type = 0;
    pub const D: Type = 1;
}
