#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum A {
    A0 = 0,
    A1 = 1,
    A2 = 2,
    A3 = -1,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum B {
    B0 = 1,
    B1 = 4,
    B2 = 3,
    B3 = -1,
}
