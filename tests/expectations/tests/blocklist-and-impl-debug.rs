#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub struct BlocklistMe(u8);

/// Because this type contains a blocklisted type, it should not derive Debug.
#[repr(C)]
pub struct ShouldManuallyImplDebug {
    pub a: BlocklistMe,
}
#[test]
fn bindgen_test_layout_ShouldManuallyImplDebug() {
    const UNINIT: ::std::mem::MaybeUninit<ShouldManuallyImplDebug> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ShouldManuallyImplDebug>(),
        1usize,
        concat!("Size of: ", stringify!(ShouldManuallyImplDebug))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldManuallyImplDebug>(),
        1usize,
        concat!("Alignment of ", stringify!(ShouldManuallyImplDebug))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldManuallyImplDebug),
            "::",
            stringify!(a)
        )
    );
}
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
