#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, unused_parens)]
pub const fn NOT(x: i64) -> i64 {
    ((((x) as i64 == 0) as i64))
}
#[allow(non_snake_case, unused_parens)]
pub const fn TOBOOL(x: i64) -> i64 {
    (((((((x) as i64 == 0) as i64)) as i64 == 0) as i64))
}
#[allow(non_snake_case, unused_parens)]
pub const fn ID(x: i64) -> i64 {
    (x)
}
#[allow(non_snake_case, unused_parens)]
pub const fn NOT_ZERO() -> u32 {
    (((0) as i64 == 0) as u32)
}
#[allow(non_snake_case, non_camel_case_types, unused_parens)]
pub const fn NOT_SIZEOF<T>() -> i64 {
    ((((core::mem::size_of::<T>() as i64) as i64 == 0) as i64))
}
#[allow(non_snake_case, unused_parens)]
pub const fn AND(x: i64, y: i64) -> i64 {
    (((((((x)) as i64) != 0) && ((((y)) as i64) != 0)) as i64))
}
#[allow(non_snake_case, unused_parens)]
pub const fn OR(x: i64, y: i64) -> i64 {
    (((((((x)) as i64) != 0) || ((((y)) as i64) != 0)) as i64))
}
#[allow(non_snake_case, unused_parens)]
pub const fn CHAIN3(a: i64, b: i64, c: i64) -> i64 {
    (((((((a)) as i64) != 0)
        && (((((((((b)) as i64) != 0) && ((((c)) as i64) != 0)) as i64)) as i64) != 0))
        as i64))
}
#[allow(non_snake_case, unused_parens)]
pub const fn MIXED_CHAIN(a: i64, b: i64, c: i64) -> i64 {
    (((((((a)) as i64) != 0)
        || (((((((((b)) as i64) != 0) && ((((c)) as i64) != 0)) as i64)) as i64) != 0))
        as i64))
}
#[allow(non_snake_case, unused_parens)]
pub const fn NOT_ID(x: i64) -> i64 {
    (((ID(x)) as i64 == 0) as i64)
}
