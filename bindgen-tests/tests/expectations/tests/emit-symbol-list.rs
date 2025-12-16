#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type my_int_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct my_struct {
    pub x: ::std::os::raw::c_int,
    pub y: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of my_struct"][::std::mem::size_of::<my_struct>() - 8usize];
    ["Alignment of my_struct"][::std::mem::align_of::<my_struct>() - 4usize];
    ["Offset of field: my_struct::x"]
        [::std::mem::offset_of!(my_struct, x) - 0usize];
    ["Offset of field: my_struct::y"]
        [::std::mem::offset_of!(my_struct, y) - 4usize];
};
unsafe extern "C" {
    pub fn my_function(arg: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub static mut my_global_var: ::std::os::raw::c_int;
}
