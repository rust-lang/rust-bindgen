#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type Foo_ctype = ::std::os::raw::c_int;
pub const Foo_Bar: Foo_ctype = -2;
pub const Foo_Qux: Foo_ctype = 1;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Foo {
    Bar = -2,
    Qux = 1,
}
