#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const y: u32 = 2;
#[doc(hidden)]
#[macro_export]
macro_rules! __cmacro__f3 {
    ($x:expr) => {
        4
    };
}
pub use __cmacro__f3 as f3;
pub const x: u32 = 2;
