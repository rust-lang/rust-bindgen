#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type A_ctype = ::std::os::raw::c_int;
pub const A_A0: A_ctype = 0;
pub const A_A1: A_ctype = 1;
pub const A_A2: A_ctype = 2;
pub const A_A3: A_ctype = -1;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum A {
    A0 = 0,
    A1 = 1,
    A2 = 2,
    A3 = -1,
}
pub type B_ctype = ::std::os::raw::c_int;
pub const B_B0: B_ctype = 1;
pub const B_B1: B_ctype = 4;
pub const B_B2: B_ctype = 3;
pub const B_B3: B_ctype = -1;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum B {
    B0 = 1,
    B1 = 4,
    B2 = 3,
    B3 = -1,
}
