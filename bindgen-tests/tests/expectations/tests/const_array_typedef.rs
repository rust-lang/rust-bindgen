#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct strct {
    pub field: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<strct>() == 4usize, "Size of strct");
    assert!(::std::mem::align_of::<strct>() == 4usize, "Alignment of strct");
    assert!(
        ::std::mem::offset_of!(strct, field) == 0usize,
        "Offset of field: strct::field",
    );
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
