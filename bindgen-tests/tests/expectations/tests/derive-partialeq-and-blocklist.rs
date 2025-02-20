#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub struct BlocklistMe(u8);
/// Because this type contains a blocklisted type, it should not derive
/// PartialEq.
#[repr(C)]
pub struct ShouldNotDerivePartialEq {
    pub a: BlocklistMe,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ShouldNotDerivePartialEq",
    ][::std::mem::size_of::<ShouldNotDerivePartialEq>() - 1usize];
    [
        "Alignment of ShouldNotDerivePartialEq",
    ][::std::mem::align_of::<ShouldNotDerivePartialEq>() - 1usize];
    [
        "Offset of field: ShouldNotDerivePartialEq::a",
    ][::std::mem::offset_of!(ShouldNotDerivePartialEq, a) - 0usize];
};
impl Default for ShouldNotDerivePartialEq {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
