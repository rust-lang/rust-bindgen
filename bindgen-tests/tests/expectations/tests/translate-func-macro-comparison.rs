#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, unused_parens)]
pub const fn GT(x: i64, y: i64) -> i64 {
    (((x) > (y)) as i64)
}
#[allow(non_snake_case, unused_parens)]
pub const fn LT(x: i64, y: i64) -> i64 {
    (((x) < (y)) as i64)
}
#[allow(non_snake_case, unused_parens)]
pub const fn EQ(x: i64, y: i64) -> i64 {
    (((x) == (y)) as i64)
}
#[allow(non_snake_case, unused_parens)]
pub const fn NE(x: i64, y: i64) -> i64 {
    (((x) != (y)) as i64)
}
#[allow(non_snake_case, unused_parens)]
pub const fn GTE(x: i64, y: i64) -> i64 {
    (((x) >= (y)) as i64)
}
#[allow(non_snake_case, unused_parens)]
pub const fn LTE(x: i64, y: i64) -> i64 {
    (((x) <= (y)) as i64)
}
#[allow(non_snake_case, unused_parens)]
pub const fn GT_BARE(x: i64, y: i64) -> i64 {
    ((x) > (y)) as i64
}
#[allow(non_snake_case, unused_parens)]
pub const fn NESTED(x: i64, y: i64) -> i64 {
    ((((((x) > (y)) as i64))))
}
#[allow(non_snake_case, unused_parens)]
pub const fn CMP_ADD(x: i64, y: i64) -> i64 {
    (((x) > (y)) as i64) + (((x) < (y)) as i64)
}
#[allow(non_snake_case, unused_parens)]
pub const fn CMP_MUL(x: i64, y: i64) -> i64 {
    ((((x) > (y)) as i64) * 2)
}
#[allow(non_snake_case, non_camel_case_types, unused_parens)]
pub const fn IS_BIG<T>() -> i64 {
    ((core::mem::size_of::<T>() as i64 > 4) as i64)
}
