#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct std_allocator_traits {
    pub _address: u8,
}
pub type std_allocator_traits___size_type<_Alloc> = _Alloc;
