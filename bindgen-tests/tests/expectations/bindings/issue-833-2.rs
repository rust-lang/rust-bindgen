#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

// If the output of this changes, please ensure issue-833-1.hpp changes too

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct nsTArray {
    pub _address: u8,
}
