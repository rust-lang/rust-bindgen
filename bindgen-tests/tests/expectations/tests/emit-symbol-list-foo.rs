pub type foo_int_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct foo_struct {
    pub x: ::std::os::raw::c_int,
    pub y: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of foo_struct"][::std::mem::size_of::<foo_struct>() - 8usize];
    ["Alignment of foo_struct"][::std::mem::align_of::<foo_struct>() - 4usize];
    ["Offset of field: foo_struct::x"][::std::mem::offset_of!(foo_struct, x) - 0usize];
    ["Offset of field: foo_struct::y"][::std::mem::offset_of!(foo_struct, y) - 4usize];
};
unsafe extern "C" {
    pub fn foo_function(arg: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub static mut foo_global_var: ::std::os::raw::c_int;
}
