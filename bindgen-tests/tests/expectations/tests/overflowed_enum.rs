#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Foo {
    BAP_ARM = 0x93fcb9,
    BAP_X86 = 0xb67eed,
    BAP_X86_64 = 0xba7b274f,
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Bar {
    One = 1,
    Big = 2,
}
