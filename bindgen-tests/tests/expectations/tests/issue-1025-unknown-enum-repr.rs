#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct a {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct b {
    pub _address: u8,
}
pub const b_SOME_VARIANT: b__bindgen_ty_1 = 0;
pub type b__bindgen_ty_1 = i32;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct c {
    pub _address: u8,
}
pub const c_my_enum_MY_ENUM_SOME_VARIANT: c_my_enum = 0;
pub type c_my_enum = i32;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct d {
    pub _address: u8,
}
pub type d_no_variant_enum = i32;
