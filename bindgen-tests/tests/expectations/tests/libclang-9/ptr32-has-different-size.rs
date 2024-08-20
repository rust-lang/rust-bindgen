#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TEST_STRUCT {
    pub ptr_32bit: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of TEST_STRUCT"][::std::mem::size_of::<TEST_STRUCT>() - 8usize];
    ["Alignment of TEST_STRUCT"][::std::mem::align_of::<TEST_STRUCT>() - 8usize];
    [
        "Offset of field: TEST_STRUCT::ptr_32bit",
    ][::std::mem::offset_of!(TEST_STRUCT, ptr_32bit) - 0usize];
};
impl Default for TEST_STRUCT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type TEST = TEST_STRUCT;
