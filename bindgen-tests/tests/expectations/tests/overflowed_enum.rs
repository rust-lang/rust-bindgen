#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type Foo_ctype = ::std::os::raw::c_uint;
pub const Foo_BAP_ARM: Foo_ctype = 9698489;
pub const Foo_BAP_X86: Foo_ctype = 11960045;
pub const Foo_BAP_X86_64: Foo_ctype = 3128633167;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Foo {
    BAP_ARM = 9698489,
    BAP_X86 = 11960045,
    BAP_X86_64 = 3128633167,
}
pub type Bar_ctype = ::std::os::raw::c_ushort;
pub const Bar_One: Bar_ctype = 1;
pub const Bar_Big: Bar_ctype = 2;
#[repr(u16)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Bar {
    One = 1,
    Big = 2,
}
