#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub struct BlocklistMe(u8);
/// Because this type contains a blocklisted type, it should not derive Copy.
#[repr(C)]
pub struct ShouldNotBeCopy {
    pub a: BlocklistMe,
}
const _: () = {
    assert!(
        ::std::mem::size_of::<ShouldNotBeCopy>() == 1usize,
        "Size of ShouldNotBeCopy",
    );
    assert!(
        ::std::mem::align_of::<ShouldNotBeCopy>() == 1usize,
        "Alignment of ShouldNotBeCopy",
    );
    assert!(
        ::std::mem::offset_of!(ShouldNotBeCopy, a) == 0usize,
        "Offset of field: ShouldNotBeCopy::a",
    );
};
impl Default for ShouldNotBeCopy {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
