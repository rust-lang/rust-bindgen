#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/** This is a forward declared struct alias with overlapping names
 and documentation.*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Struct {
    _unused: [u8; 0],
}
