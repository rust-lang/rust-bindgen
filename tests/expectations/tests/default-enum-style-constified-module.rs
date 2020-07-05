#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub mod Foo {
    pub type Type = u32;
    pub const bar: Type = 0;
    pub const baz: Type = 1;
    pub const blap: Type = 2;
}
extern "C" {
    pub fn func(x: Foo::Type);
}
