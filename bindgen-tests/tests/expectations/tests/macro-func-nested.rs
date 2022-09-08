#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[doc(hidden)]
#[macro_export]
macro_rules! __cmacro__f1 {
    ($x:expr) => {
        $x * 2
    };
}
pub use __cmacro__f1 as f1;
#[doc(hidden)]
#[macro_export]
macro_rules! __cmacro__f2 {
    ($y:expr) => {
        $y * $y * 2
    };
}
pub use __cmacro__f2 as f2;
