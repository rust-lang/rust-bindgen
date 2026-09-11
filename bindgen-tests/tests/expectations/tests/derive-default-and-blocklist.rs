#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub struct BlocklistMe(u8);
/// Because this type contains a blocklisted type, it should not derive
/// Default. Instead, we should emit a `mem::zeroed` implementation.
#[repr(C)]
pub struct ShouldNotDeriveDefault {
    pub a: BlocklistMe,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ShouldNotDeriveDefault",
    ][::std::mem::size_of::<ShouldNotDeriveDefault>() - 1usize];
    [
        "Alignment of ShouldNotDeriveDefault",
    ][::std::mem::align_of::<ShouldNotDeriveDefault>() - 1usize];
    [
        "Offset of field: ShouldNotDeriveDefault::a",
    ][::std::mem::offset_of!(ShouldNotDeriveDefault, a) - 0usize];
};
impl Default for ShouldNotDeriveDefault {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
