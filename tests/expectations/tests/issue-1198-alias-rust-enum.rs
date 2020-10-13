#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

impl MyDupeEnum {
    pub const A_alias: MyDupeEnum = MyDupeEnum::A;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MyDupeEnum {
    A = 0,
    B = 1,
}
impl MyOtherDupeEnum {
    pub const C_alias: MyOtherDupeEnum = MyOtherDupeEnum::C;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MyOtherDupeEnum {
    C = 0,
    D = 1,
}
