#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// <div rustbindgen attribute="#[derive(Debug)]"></div>
#[repr(C)]
#[derive(Debug)]
pub struct my_type {
    pub a: ::std::os::raw::c_int,
}
/// <div rustbindgen attribute="#[derive(Debug)]"></div>
/// <div rustbindgen attribute="#[derive(Clone)]"></div>
#[repr(C)]
#[derive(Debug)]
#[derive(Clone)]
pub struct my_type2 {
    pub a: ::std::os::raw::c_uint,
}
/// <div rustbindgen attribute="#[derive(Debug)]" attribute="#[derive(Clone)]"></div>
#[repr(C)]
#[derive(Debug)]
#[derive(Clone)]
pub struct my_type3 {
    pub a: ::std::os::raw::c_ulong,
}
