#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, unused_parens)]
pub const fn BOOL_NOT(x: i64) -> i64 {
    (if (((((x) as i64 == 0) as i64)) as i64) != 0 { 1 } else { 0 })
}
#[allow(non_snake_case, unused_parens)]
pub const fn BOOL_AND(x: i64, y: i64) -> i64 {
    (if ((((((((x)) as i64) != 0) && ((((y)) as i64) != 0)) as i64)) as i64) != 0 {
        1
    } else {
        0
    })
}
#[allow(non_snake_case, unused_parens)]
pub const fn BOOL_OR(x: i64, y: i64) -> i64 {
    (if ((((((((x)) as i64) != 0) || ((((y)) as i64) != 0)) as i64)) as i64) != 0 {
        1
    } else {
        0
    })
}
#[allow(non_snake_case, unused_parens)]
pub const fn CMP_GT(x: i64, y: i64) -> i64 {
    (if (x) > (y) { (x) } else { (y) })
}
#[allow(non_snake_case, unused_parens)]
pub const fn RANGE(x: i64) -> i64 {
    (if (((((x) > 0 && (x) < 10) as i64)) as i64) != 0 { 1 } else { 0 })
}
#[allow(non_snake_case, unused_parens)]
pub const fn ALL3(x: i64, y: i64, z: i64) -> i64 {
    (if (((((x) > 0 && ((((((y) > 0 && (z) > 0) as i64)) as i64) != 0)) as i64)) as i64)
        != 0
    {
        1
    } else {
        0
    })
}
#[allow(non_snake_case, unused_parens)]
pub const fn ANY3(x: i64, y: i64, z: i64) -> i64 {
    (if (((((x) > 0 || ((((((y) > 0 || (z) > 0) as i64)) as i64) != 0)) as i64)) as i64)
        != 0
    {
        1
    } else {
        0
    })
}
#[allow(non_snake_case, unused_parens)]
pub const fn MIXED3(x: i64, y: i64, z: i64) -> i64 {
    (if (((((x) > 0 || ((((((y) > 0 && (z) > 0) as i64)) as i64) != 0)) as i64)) as i64)
        != 0
    {
        1
    } else {
        0
    })
}
