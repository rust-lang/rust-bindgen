#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type EmptyConstified = ::std::os::raw::c_uint;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EmptyRustified {
    __bindgen_cannot_repr_c_on_empty_enum = 0,
}
pub mod EmptyModule {
    pub type Type = ::std::os::raw::c_uint;
}
#[repr(i8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EmptyClassRustified {
    __bindgen_cannot_repr_c_on_empty_enum = 0,
}
pub type EmptyClassConstified = ::std::os::raw::c_char;
pub mod EmptyClassModule {
    pub type Type = ::std::os::raw::c_char;
}
#[repr(i8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ForwardClassRustified {
    __bindgen_cannot_repr_c_on_empty_enum = 0,
}
pub type ForwardClassConstified = ::std::os::raw::c_char;
pub mod ForwardClassModule {
    pub type Type = ::std::os::raw::c_char;
}
