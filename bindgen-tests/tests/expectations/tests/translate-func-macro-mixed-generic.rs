#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, unused_parens)]
pub const fn M<T>(x: i64) -> i64 {
    (core::mem::size_of::<T>() as i64 + (x))
}
#[allow(non_snake_case, non_camel_case_types, unused_parens)]
pub const fn N<T>(y: i64) -> i64 {
    (M::<T>(y) as i64)
}
