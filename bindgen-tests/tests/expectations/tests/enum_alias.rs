#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type Bar_ctype = u8;
pub const Bar_VAL: Bar_ctype = 0;
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Bar {
    VAL = 0,
}
