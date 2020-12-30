#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub mod B {
    ///```text
    /// Document enum
    ///```
    pub type Type = ::std::os::raw::c_uint;
    ///```text
    /// Document field with three slashes
    ///```
    pub const VAR_A: Type = 0;
    ///```text
    /// Document field with preceeding star
    ///```
    pub const VAR_B: Type = 1;
    ///```text
    /// Document field with preceeding exclamation
    ///```
    pub const VAR_C: Type = 2;
    ///```text
    ///< Document field with following star
    ///```
    pub const VAR_D: Type = 3;
    ///```text
    ///< Document field with following exclamation
    ///```
    pub const VAR_E: Type = 4;
    ///```text
    /// Document field with preceeding star, with a loong long multiline
    /// comment.
    ///
    /// Very interesting documentation, definitely.
    ///```
    pub const VAR_F: Type = 5;
}
