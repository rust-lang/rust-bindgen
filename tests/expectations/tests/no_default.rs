#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

///```text
/// <div rustbindgen nodefault></div>
///```
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DefaultButWait {
    pub whatever: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DefaultButWaitDerived {
    pub whatever: DefaultButWait,
}
impl Default for DefaultButWaitDerived {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
