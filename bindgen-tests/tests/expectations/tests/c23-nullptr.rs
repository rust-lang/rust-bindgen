#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type nullptr_t = *const ::std::os::raw::c_void;
pub type NullAlias = nullptr_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NullHolder {
    pub value: nullptr_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of NullHolder"][::std::mem::size_of::<NullHolder>() - 8usize];
    ["Alignment of NullHolder"][::std::mem::align_of::<NullHolder>() - 8usize];
    [
        "Offset of field: NullHolder::value",
    ][::std::mem::offset_of!(NullHolder, value) - 0usize];
};
impl Default for NullHolder {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn ident(value: nullptr_t) -> nullptr_t;
}
unsafe extern "C" {
    pub fn is_null(value: NullAlias) -> ::std::os::raw::c_int;
}
