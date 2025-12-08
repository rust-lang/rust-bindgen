#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Base {
    pub large: [::std::os::raw::c_int; 33usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Base"][::std::mem::size_of::<Base>() - 132usize];
    ["Alignment of Base"][::std::mem::align_of::<Base>() - 4usize];
    ["Offset of field: Base::large"][::std::mem::offset_of!(Base, large) - 0usize];
};
impl Default for Base {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShouldDerivePartialEq {
    pub _base: Base,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ShouldDerivePartialEq",
    ][::std::mem::size_of::<ShouldDerivePartialEq>() - 132usize];
    [
        "Alignment of ShouldDerivePartialEq",
    ][::std::mem::align_of::<ShouldDerivePartialEq>() - 4usize];
};
impl Default for ShouldDerivePartialEq {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
