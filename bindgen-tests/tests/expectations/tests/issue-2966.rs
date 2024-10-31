#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct pub_var1(pub *const ::std::os::raw::c_char);
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct pubcrate_var2(pub(crate) *const ::std::os::raw::c_char);
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct private_var3(*const ::std::os::raw::c_char);
