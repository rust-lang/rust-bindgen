#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug)]
pub struct JS_Zone {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct JS_shadow_Zone {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of JS_shadow_Zone"][::std::mem::size_of::<JS_shadow_Zone>() - 8usize];
    ["Alignment of JS_shadow_Zone"][::std::mem::align_of::<JS_shadow_Zone>() - 4usize];
    [
        "Offset of field: JS_shadow_Zone::x",
    ][::std::mem::offset_of!(JS_shadow_Zone, x) - 0usize];
    [
        "Offset of field: JS_shadow_Zone::y",
    ][::std::mem::offset_of!(JS_shadow_Zone, y) - 4usize];
};
