#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct std_fbstring_core {
    pub _address: u8,
}
pub type std_fbstring_core_category_type = u8;
impl std_fbstring_core_Category {
    pub const Bar: std_fbstring_core_Category = std_fbstring_core_Category::Foo;
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum std_fbstring_core_Category {
    Foo = 0,
}
