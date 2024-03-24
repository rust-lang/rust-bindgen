#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(transparent)]
pub struct __BindgenFloat16(pub u16);
extern "C" {
    pub static mut global: __BindgenFloat16;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Test__Float16 {
    pub f: __BindgenFloat16,
}
const _: () = {
    ["Size of Test__Float16"][::std::mem::size_of::<Test__Float16>() - 2usize];
    ["Alignment of Test__Float16"][::std::mem::align_of::<Test__Float16>() - 2usize];
    [
        "Offset of field: Test__Float16::f",
    ][::std::mem::offset_of!(Test__Float16, f) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Test__Float16Ref {
    pub f: *mut __BindgenFloat16,
}
const _: () = {
    ["Size of Test__Float16Ref"][::std::mem::size_of::<Test__Float16Ref>() - 8usize];
    [
        "Alignment of Test__Float16Ref",
    ][::std::mem::align_of::<Test__Float16Ref>() - 8usize];
    [
        "Offset of field: Test__Float16Ref::f",
    ][::std::mem::offset_of!(Test__Float16Ref, f) - 0usize];
};
impl Default for Test__Float16Ref {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
