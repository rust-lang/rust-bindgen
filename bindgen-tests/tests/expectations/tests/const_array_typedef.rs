#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct strct {
    pub field: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of strct"][::std::mem::size_of::<strct>() - 4usize];
    ["Alignment of strct"][::std::mem::align_of::<strct>() - 4usize];
    ["Offset of field: strct::field"][::std::mem::offset_of!(strct, field) - 0usize];
};
pub type typ = [strct; 1usize];
extern "C" {
    pub static mut w: typ;
}
extern "C" {
    pub static mut x: *mut strct;
}
extern "C" {
    pub static y: typ;
}
extern "C" {
    pub static mut z: *const strct;
}
extern "C" {
    pub fn function(a: *const strct, b: *const strct);
}
