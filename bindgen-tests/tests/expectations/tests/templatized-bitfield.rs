#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// We don't get a layout for this bitfield, since we don't know what `T` will
/// be, so we cannot allocate bitfield units. The best thing we can do is make
/// the struct opaque.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TemplatizedBitfield {
    pub _address: u8,
}
