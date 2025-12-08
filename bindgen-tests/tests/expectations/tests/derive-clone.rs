#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// This struct should derive `Clone`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ShouldDeriveClone {
    pub large: [::std::os::raw::c_int; 33usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ShouldDeriveClone"][::std::mem::size_of::<ShouldDeriveClone>() - 132usize];
    [
        "Alignment of ShouldDeriveClone",
    ][::std::mem::align_of::<ShouldDeriveClone>() - 4usize];
    [
        "Offset of field: ShouldDeriveClone::large",
    ][::std::mem::offset_of!(ShouldDeriveClone, large) - 0usize];
};
impl Default for ShouldDeriveClone {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
