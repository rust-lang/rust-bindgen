#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const BEFORE_DECL: *mut later = 3u64 as usize as *mut later;
pub const CONST_PTR: *const later = 4u64 as usize as *const later;
pub const TYPEDEF_PTR: *mut later = 5u64 as usize as *mut later;
pub const MAP_FAILED: *mut ::std::os::raw::c_void = 18446744073709551615u64 as usize
    as *mut ::std::os::raw::c_void;
pub const MAP_FAILED_ALIAS: *mut ::std::os::raw::c_void = 18446744073709551615u64
    as usize as *mut ::std::os::raw::c_void;
pub const MAP_FAILED_EQUALS_ITSELF: u32 = 1;
pub const REDEFINED_FROM_INT: u32 = 1;
pub const REDEFINED_ALIAS: *mut ::std::os::raw::c_void = 2u64 as usize
    as *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug)]
pub struct later {
    _unused: [u8; 0],
}
pub type later_ptr = *mut later;
