#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub struct BlocklistMe(u8);
/// Because this type contains a blocklisted type, it should not derive Debug.
#[repr(C)]
pub struct ShouldManuallyImplDebug {
    pub a: BlocklistMe,
}
const _: () = {
    assert!(
        ::std::mem::size_of::<ShouldManuallyImplDebug>() == 1usize,
        "Size of ShouldManuallyImplDebug",
    );
    assert!(
        ::std::mem::align_of::<ShouldManuallyImplDebug>() == 1usize,
        "Alignment of ShouldManuallyImplDebug",
    );
    assert!(
        ::std::mem::offset_of!(ShouldManuallyImplDebug, a) == 0usize,
        "Offset of field: ShouldManuallyImplDebug::a",
    );
};
impl Default for ShouldManuallyImplDebug {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for ShouldManuallyImplDebug {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "ShouldManuallyImplDebug {{  }}")
    }
}
