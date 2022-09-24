#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Foo {
    BAP_ARM = 9698489,
    BAP_X86 = 11960045,
    BAP_X86_64 = 3128633167,
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Bar {
    One = 1,
    Big = 2,
}
