#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
impl MyDupeEnum {
    pub const A_alias: MyDupeEnum = MyDupeEnum::A;
}
pub type MyDupeEnum_ctype = ::std::os::raw::c_uint;
pub const MyDupeEnum_A: MyDupeEnum_ctype = 0;
pub const MyDupeEnum_B: MyDupeEnum_ctype = 1;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MyDupeEnum {
    A = 0,
    B = 1,
}
impl MyOtherDupeEnum {
    pub const C_alias: MyOtherDupeEnum = MyOtherDupeEnum::C;
}
pub type MyOtherDupeEnum_ctype = ::std::os::raw::c_uint;
pub const MyOtherDupeEnum_C: MyOtherDupeEnum_ctype = 0;
pub const MyOtherDupeEnum_D: MyOtherDupeEnum_ctype = 1;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MyOtherDupeEnum {
    C = 0,
    D = 1,
}
