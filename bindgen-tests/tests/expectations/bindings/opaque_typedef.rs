#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct RandomTemplate {
    pub _address: u8,
}
/// <div rustbindgen opaque></div>
pub type ShouldBeOpaque = u8;
pub type ShouldNotBeOpaque = RandomTemplate;
