#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const foo: _bindgen_ty_1 = _bindgen_ty_1::foo;
pub const bar: _bindgen_ty_1 = _bindgen_ty_1::bar;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_1 {
    foo = 4,
    bar = 8,
}
pub type EasyToOverflow = ::std::os::raw::c_ulonglong;
pub const k: EasyToOverflow = 2147483648;
pub const k_expr: EasyToOverflow = 1152921504606846976;
pub const wow: EasyToOverflow = 2147483648;
pub const BAZ: ::std::os::raw::c_longlong = 24;
pub const fuzz: f64 = 51.0;
pub const BAZZ: ::std::os::raw::c_char = 53;
pub const WAT: ::std::os::raw::c_char = 0;
pub const bytestring: &[u8; 4] = b"Foo\0";
pub const NOT_UTF8: &[u8; 5] = b"\xF0(\x8C(\0";
