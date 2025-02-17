#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod B {
    /// Document enum
    pub type Type = ::std::os::raw::c_uint;
    /// Document field with three slashes
    pub const VAR_A: Type = 0;
    /// Document field with preceding star
    pub const VAR_B: Type = 1;
    /// Document field with preceding exclamation
    pub const VAR_C: Type = 2;
    ///< Document field with following star
    pub const VAR_D: Type = 3;
    ///< Document field with following exclamation
    pub const VAR_E: Type = 4;
    /// Document field with preceding star, with a loong long multiline
    /// comment.
    ///
    /// Very interesting documentation, definitely.
    pub const VAR_F: Type = 5;
}
