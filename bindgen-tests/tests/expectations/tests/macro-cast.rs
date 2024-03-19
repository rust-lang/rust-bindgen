#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const ULONG_MAX: u32 = 4294967295;
pub type TickType_t = ::std::os::raw::c_ulong;
pub const portMAX_DELAY: TickType_t = 4294967295u32 as TickType_t;
pub const NEG_TO_POS: ::std::os::raw::c_uint = -1i8 as ::std::os::raw::c_uint;
pub const BIG_TO_SMALL: ::std::os::raw::c_ushort = 4294967295u32
    as ::std::os::raw::c_ushort;
