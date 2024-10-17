#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type B_ctype = ::std::os::raw::c_uint;
/// Document field with three slashes
pub const B_VAR_A: B_ctype = 0;
/// Document field with preceding star
pub const B_VAR_B: B_ctype = 1;
/// Document field with preceding exclamation
pub const B_VAR_C: B_ctype = 2;
///< Document field with following star
pub const B_VAR_D: B_ctype = 3;
///< Document field with following exclamation
pub const B_VAR_E: B_ctype = 4;
/** Document field with preceding star, with a loong long multiline
 comment.

 Very interesting documentation, definitely.*/
pub const B_VAR_F: B_ctype = 5;
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
    /** Document field with preceding star, with a loong long multiline
 comment.

 Very interesting documentation, definitely.*/
    VAR_F = 5,
}
