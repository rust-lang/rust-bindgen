#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// This is Struct
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Struct {
    /// This is field
    pub field: ::std::os::raw::c_int,
}
/// This is AliasToStruct
pub type AliasToStruct = Struct;
/// This is AliasToInt
pub type AliasToInt = ::std::os::raw::c_int;
/// This is AliasToAliasToInt
pub type AliasToAliasToInt = AliasToInt;
