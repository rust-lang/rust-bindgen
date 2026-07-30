#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type test_AliasUnion = test_RecursiveUnion;
#[repr(C)]
#[derive(Copy, Clone)]
pub union test_RecursiveUnion {
    pub x: ::std::os::raw::c_int,
    pub self_ptr: *mut test_AliasUnion,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of test_RecursiveUnion",
    ][::std::mem::size_of::<test_RecursiveUnion>() - 8usize];
    [
        "Alignment of test_RecursiveUnion",
    ][::std::mem::align_of::<test_RecursiveUnion>() - 8usize];
    [
        "Offset of field: test_RecursiveUnion::x",
    ][::std::mem::offset_of!(test_RecursiveUnion, x) - 0usize];
    [
        "Offset of field: test_RecursiveUnion::self_ptr",
    ][::std::mem::offset_of!(test_RecursiveUnion, self_ptr) - 0usize];
};
impl Default for test_RecursiveUnion {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
