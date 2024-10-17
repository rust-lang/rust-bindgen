#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type Foo_ctype = ::std::os::raw::c_uchar;
pub const Foo_Bar: Foo_ctype = 0;
pub const Foo_Qux: Foo_ctype = 1;
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Foo {
    Bar = 0,
    Qux = 1,
}
pub type Neg_ctype = ::std::os::raw::c_schar;
pub const Neg_MinusOne: Neg_ctype = -1;
pub const Neg_One: Neg_ctype = 1;
#[repr(i8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Neg {
    MinusOne = -1,
    One = 1,
}
pub type Bigger_ctype = ::std::os::raw::c_ushort;
pub const Bigger_Much: Bigger_ctype = 255;
pub const Bigger_Larger: Bigger_ctype = 256;
#[repr(u16)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Bigger {
    Much = 255,
    Larger = 256,
}
