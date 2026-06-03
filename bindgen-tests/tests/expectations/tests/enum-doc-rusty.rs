#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(u32)]
/// Document enum
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum B {
    /// Document field with three slashes
    VAR_A = 0,
    /// Document field with preceding star
    VAR_B = 1,
    /// Document field with preceding exclamation
    VAR_C = 2,
    ///< Document field with following star
    VAR_D = 3,
    ///< Document field with following exclamation
    VAR_E = 4,
    /// Document field with preceding star, with a loong long multiline
    /// comment.
    ///
    /// Very interesting documentation, definitely.
    VAR_F = 5,
}
