#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// Document field with three slashes
pub const B_VAR_A: B = 0;
/// Document field with preceding star
pub const B_VAR_B: B = 1;
/// Document field with preceding exclamation
pub const B_VAR_C: B = 2;
///< Document field with following star
pub const B_VAR_D: B = 3;
///< Document field with following exclamation
pub const B_VAR_E: B = 4;
/// Document field with preceding star, with a loong long multiline
/// comment.
///
/// Very interesting documentation, definitely.
pub const B_VAR_F: B = 5;
/// Document enum
pub type B = ::std::os::raw::c_uint;
