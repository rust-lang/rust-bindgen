#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// <div rustbindgen derive="Debug"></div>
#[repr(C)]
#[derive(Default, Debug)]
pub struct my_type {
    pub a: ::std::os::raw::c_int,
}
/// <div rustbindgen derive="Debug"></div>
/// <div rustbindgen derive="Clone"></div>
#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct my_type2 {
    pub a: ::std::os::raw::c_uint,
}
/// <div rustbindgen derive="Debug" derive="Clone"></div>
#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct my_type3 {
    pub a: ::std::os::raw::c_ulong,
}
