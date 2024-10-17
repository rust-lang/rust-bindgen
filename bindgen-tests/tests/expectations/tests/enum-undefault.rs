#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type Foo_ctype = ::std::os::raw::c_uint;
pub const Foo_Bar: Foo_ctype = 0;
pub const Foo_Qux: Foo_ctype = 1;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Foo {
    Bar = 0,
    Qux = 1,
}
pub const Neg_MinusOne: Neg = -1;
pub const Neg_One: Neg = 1;
pub type Neg = ::std::os::raw::c_int;
