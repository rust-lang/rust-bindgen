#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const SHIFT: u32 = 8;
#[allow(non_snake_case, unused_parens)]
pub const fn USE_SHIFT(x: u32) -> u32 {
    ((x) << SHIFT)
}
