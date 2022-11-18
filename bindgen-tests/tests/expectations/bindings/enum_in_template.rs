#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub _address: u8,
}
pub const Foo_Bar_A: Foo_Bar = 0;
pub const Foo_Bar_B: Foo_Bar = 0;
pub type Foo_Bar = i32;
