#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub struct BlocklistMe(u8);
/// Because this type contains a blocklisted type, it should not derive Hash.
#[repr(C)]
pub struct ShouldNotDeriveHash {
    pub a: BlocklistMe,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ShouldNotDeriveHash",
    ][::std::mem::size_of::<ShouldNotDeriveHash>() - 1usize];
    [
        "Alignment of ShouldNotDeriveHash",
    ][::std::mem::align_of::<ShouldNotDeriveHash>() - 1usize];
    [
        "Offset of field: ShouldNotDeriveHash::a",
    ][::std::mem::offset_of!(ShouldNotDeriveHash, a) - 0usize];
};
impl Default for ShouldNotDeriveHash {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
