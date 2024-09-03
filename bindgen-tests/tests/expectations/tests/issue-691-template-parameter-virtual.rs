#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
pub struct VirtualMethods__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VirtualMethods {
    pub vtable_: *const VirtualMethods__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VirtualMethods"][::std::mem::size_of::<VirtualMethods>() - 8usize];
    ["Alignment of VirtualMethods"][::std::mem::align_of::<VirtualMethods>() - 8usize];
};
impl Default for VirtualMethods {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Set {
    pub bar: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ServoElementSnapshotTable {
    pub _base: Set,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ServoElementSnapshotTable",
    ][::std::mem::size_of::<ServoElementSnapshotTable>() - 4usize];
    [
        "Alignment of ServoElementSnapshotTable",
    ][::std::mem::align_of::<ServoElementSnapshotTable>() - 4usize];
};
impl Default for ServoElementSnapshotTable {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Set_open0_VirtualMethods_close0",
    ][::std::mem::size_of::<Set>() - 4usize];
    [
        "Align of template specialization: Set_open0_VirtualMethods_close0",
    ][::std::mem::align_of::<Set>() - 4usize];
};
