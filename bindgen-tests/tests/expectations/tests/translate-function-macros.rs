#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const BIG: u32 = 42;
#[allow(non_snake_case, unused_parens)]
pub const fn ADD(x: i64, y: i64) -> i64 {
    ((x) + (y))
}
#[allow(non_snake_case, unused_parens)]
pub const fn DOUBLE(x: i64) -> i64 {
    ((x) * 2)
}
#[allow(non_snake_case, unused_parens)]
pub const fn FLAG(n: i64) -> i64 {
    (1 << (n))
}
#[allow(non_snake_case, unused_parens)]
pub const fn COMPLEMENT(x: i64) -> i64 {
    (!(x))
}
#[allow(non_snake_case, unused_parens)]
pub const fn MAX(a: i64, b: i64) -> i64 {
    (if ((((a) > (b)) as i64) as i64) != 0 { (a) } else { (b) })
}
#[allow(non_snake_case, unused_parens)]
pub const fn ABS(x: i64) -> i64 {
    (if ((((x) >= 0) as i64) as i64) != 0 { (x) } else { -(x) })
}
#[allow(non_snake_case, unused_parens)]
pub const fn TO_UNSIGNED(x: i64) -> i64 {
    (((x) as u32 as i64))
}
#[allow(non_snake_case, non_camel_case_types, unused_parens)]
pub const fn SIZEOF_T<T>() -> i64 {
    core::mem::size_of::<T>() as i64
}
#[allow(non_snake_case, unused_parens)]
pub const fn IOC(dir: i64, r#type: i64, nr: i64, size: i64) -> i64 {
    (((dir) << 30) | ((r#type) << 8) | (nr) | ((size) << 16))
}
#[allow(non_snake_case, unused_parens)]
pub const fn SHIFT(r#type: i64, n: i64) -> i64 {
    ((r#type) << (n))
}
#[allow(non_snake_case, unused_parens)]
pub const fn MASK(n: u64) -> u64 {
    (0xFF << (n))
}
#[allow(non_snake_case, unused_parens)]
pub const fn IO(r#type: i64, nr: i64) -> i64 {
    IOC(0, (r#type), (nr), 0)
}
