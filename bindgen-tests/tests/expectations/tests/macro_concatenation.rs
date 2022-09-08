#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[doc(hidden)]
#[macro_export]
macro_rules! __cmacro__UL {
    ($X:ident) => {
        ::std::concat_idents!($X, UL)
    };
}
pub use __cmacro__UL as UL;
pub const ONE: u32 = 1;
