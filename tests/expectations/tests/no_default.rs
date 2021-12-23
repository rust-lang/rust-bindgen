#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// <div rustbindgen nodefault></div>
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
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
