#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

///```text
/// Document field with three slashes
///```
pub const B_VAR_A: B = 0;
///```text
/// Document field with preceeding star
///```
pub const B_VAR_B: B = 1;
///```text
/// Document field with preceeding exclamation
///```
pub const B_VAR_C: B = 2;
///```text
///< Document field with following star
///```
pub const B_VAR_D: B = 3;
///```text
///< Document field with following exclamation
///```
pub const B_VAR_E: B = 4;
///```text
/// Document field with preceeding star, with a loong long multiline
/// comment.
///
/// Very interesting documentation, definitely.
///```
pub const B_VAR_F: B = 5;
///```text
/// Document enum
///```
pub type B = ::std::os::raw::c_uint;
