#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[doc(hidden)]
#[macro_export]
macro_rules! __cmacro___IO {
    ($a_:expr) => {
        $a_
    };
}
pub use __cmacro___IO as _IO;
pub const UI_DEV_CREATE: u32 = 1;
pub const UI_DEV_DESTROY: u32 = 2;
