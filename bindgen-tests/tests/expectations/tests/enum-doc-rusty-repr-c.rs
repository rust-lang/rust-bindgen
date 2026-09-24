#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of B"][::std::mem::size_of::<B>() - 4usize];
    ["Alignment of B"][::std::mem::align_of::<B>() - 4usize];
};
#[repr(C)]
/// An enum with a value larger than the platform's `c_int`
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BigEnum {
    /// A value that is too large to fit in a 32-bit integer
    BIG_ENUM_BIG = 4294967296,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of BigEnum"][::std::mem::size_of::<BigEnum>() - 8usize];
    ["Alignment of BigEnum"][::std::mem::align_of::<BigEnum>() - 8usize];
};
