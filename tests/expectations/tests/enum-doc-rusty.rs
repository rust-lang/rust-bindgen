#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(u32)]
///```text
/// Document enum
///```
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum B {
    ///```text
    /// Document field with three slashes
    ///```
    VAR_A = 0,
    ///```text
    /// Document field with preceeding star
    ///```
    VAR_B = 1,
    ///```text
    /// Document field with preceeding exclamation
    ///```
    VAR_C = 2,
    ///```text
    ///< Document field with following star
    ///```
    VAR_D = 3,
    ///```text
    ///< Document field with following exclamation
    ///```
    VAR_E = 4,
    ///```text
    /// Document field with preceeding star, with a loong long multiline
    /// comment.
    ///
    /// Very interesting documentation, definitely.
    ///```
    VAR_F = 5,
}
