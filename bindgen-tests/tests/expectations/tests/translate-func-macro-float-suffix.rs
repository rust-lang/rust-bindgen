#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, unused_parens)]
pub const fn ADD_HALF(x: f64) -> f64 {
    ((x) + 0.5)
}
#[allow(non_snake_case, unused_parens)]
pub const fn ADD_QUARTER(x: f64) -> f64 {
    ((x) + 0.25)
}
#[allow(non_snake_case, unused_parens)]
pub const fn ADD_EXP(x: f64) -> f64 {
    ((x) + 1e3)
}
#[allow(non_snake_case, unused_parens)]
pub const fn ADD_EXPF(x: f64) -> f64 {
    ((x) + 1e3)
}
#[allow(non_snake_case, unused_parens)]
pub const fn HEX_MASK(x: i64) -> i64 {
    ((x) & 0xFF)
}
#[allow(non_snake_case, unused_parens)]
pub const fn HEX_COMBO(x: i64) -> i64 {
    ((x) | 0xABCDEF)
}
#[allow(non_snake_case, unused_parens)]
pub const fn NOT_HALF() -> f64 {
    (((0.5) == 0.0) as i64 as f64)
}
#[allow(non_snake_case, unused_parens)]
pub const fn NOT_PARAM(x: f64) -> f64 {
    (if ((((x) == 0.0) as i64 as f64)) != 0.0 { 1.0 } else { 0.0 })
}
#[allow(non_snake_case, unused_parens)]
pub const fn FLOAT_BOTH() -> f64 {
    (if (((((0.5) != 0.0) && ((2.0) != 0.0)) as i64 as f64)) != 0.0 { 1.0 } else { 0.0 })
}
#[allow(non_snake_case, unused_parens)]
pub const fn FLOAT_CHAIN() -> f64 {
    (if (((((0.5) != 0.0)
        && ((((((2.0) != 0.0) && ((3.0) != 0.0)) as i64 as f64)) != 0.0)) as i64 as f64))
        != 0.0
    {
        1.0
    } else {
        0.0
    })
}
