#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const REDEFINED_FROM_INT: u32 = 1;
#[repr(C)]
#[derive(Debug)]
pub struct later {
    _unused: [u8; 0],
}
pub type later_ptr = *mut later;
