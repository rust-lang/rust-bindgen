#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const _IOC_NRSHIFT: u32 = 0;
pub const _IOC_TYPESHIFT: u32 = 8;
pub const _IOC_SIZESHIFT: u32 = 16;
pub const _IOC_DIRSHIFT: u32 = 30;
pub const _IOC_NONE: u32 = 0;
pub const _IOC_READ: u32 = 2;
#[allow(non_snake_case, unused_parens)]
pub const fn _IOC(dir: u32, r#type: u32, nr: u32, size: u32) -> u32 {
    (((dir) << _IOC_DIRSHIFT) | ((r#type) << _IOC_TYPESHIFT) | ((nr) << _IOC_NRSHIFT)
        | ((size) << _IOC_SIZESHIFT))
}
#[allow(non_snake_case, non_camel_case_types, unused_parens)]
pub const fn _IOC_TYPECHECK<t>() -> i64 {
    (core::mem::size_of::<t>() as i64)
}
#[allow(non_snake_case, unused_parens)]
pub const fn _IO(r#type: u32, nr: u32) -> u32 {
    _IOC(_IOC_NONE, (r#type), (nr), 0)
}
#[allow(non_snake_case, non_camel_case_types, unused_parens)]
pub const fn _IOR<argtype>(r#type: u32, nr: u32) -> u32 {
    _IOC(_IOC_READ, (r#type), (nr), ((_IOC_TYPECHECK::<argtype>() as u32)))
}
