include!("emit-symbol-list-foo.rs");
pub type bar_int_t = foo_int_t;
#[repr(C)]
pub struct bar_struct {
    pub foo: foo_struct,
    pub z: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bar_struct"][::std::mem::size_of::<bar_struct>() - 12usize];
    ["Alignment of bar_struct"][::std::mem::align_of::<bar_struct>() - 4usize];
    [
        "Offset of field: bar_struct::foo",
    ][::std::mem::offset_of!(bar_struct, foo) - 0usize];
    ["Offset of field: bar_struct::z"][::std::mem::offset_of!(bar_struct, z) - 8usize];
};
unsafe extern "C" {
    pub fn bar_function(arg: *mut foo_struct, arg2: *mut bar_struct);
}
unsafe extern "C" {
    pub static mut bar_global_var: ::std::os::raw::c_int;
}
