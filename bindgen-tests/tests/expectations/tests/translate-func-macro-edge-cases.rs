#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const A: u32 = 1;
pub const B: i32 = -1;
unsafe extern "C" {
    pub static mut ext_var: ::std::os::raw::c_int;
}
#[allow(non_snake_case, unused_parens)]
pub const fn NEG() -> i64 {
    (((-1) as i32 as i64))
}
#[allow(non_snake_case, unused_parens)]
pub const fn NEGX(x: i64) -> i64 {
    (((-(x)) as i32 as i64))
}
#[allow(non_snake_case, unused_parens)]
pub const fn POS(x: f64) -> f64 {
    (if (x) > (0 as f64) { 1.0 } else { 0.0 })
}
#[allow(non_snake_case, unused_parens)]
pub const fn MIX(x: u32) -> u32 {
    ((x) + A + (B as u32))
}
#[allow(non_snake_case, unused_parens)]
pub const fn ALL_BITS() -> u32 {
    (!0)
}
#[allow(non_snake_case, unused_parens)]
pub const fn SMALL_U(x: u32) -> u32 {
    ((x) + 1)
}
#[allow(non_snake_case, unused_parens)]
pub const fn BIG_U() -> u64 {
    0x100000000
}
#[allow(non_snake_case, unused_parens)]
pub const fn BIG_OCT_U() -> u64 {
    0o40000000000
}
#[allow(non_snake_case, unused_parens)]
pub const fn HUGE(x: u64) -> u64 {
    ((x) + 0xFFFFFFFFFFFFFFFF)
}
#[allow(non_snake_case, unused_parens)]
pub const fn ALL_BITS_UL() -> u64 {
    (!0)
}
#[allow(non_snake_case, unused_parens)]
pub const fn ALL_BITS_LU() -> u64 {
    (!0)
}
#[allow(non_snake_case, unused_parens)]
pub const fn ALL_BITS_LLU() -> u64 {
    (!0)
}
#[allow(non_snake_case, unused_parens)]
pub const fn ALL_BITS_UI64() -> u64 {
    (!0)
}
#[allow(non_snake_case, unused_parens)]
pub const fn HUGE_MIXED(x: u64) -> u64 {
    ((x) + (A as u64) + (B as u64) + 0xFFFFFFFFFFFFFFFF)
}
#[allow(non_snake_case, unused_parens)]
pub const fn PERMS() -> i64 {
    0o644
}
#[allow(non_snake_case, unused_parens)]
pub const fn OCTAL(x: i64) -> i64 {
    ((x) + 0o77)
}
#[allow(non_snake_case, unused_parens)]
pub const fn GOOD(x: i64) -> i64 {
    ((x) + 1)
}
