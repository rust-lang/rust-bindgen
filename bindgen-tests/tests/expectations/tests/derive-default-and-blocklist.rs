#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub struct BlocklistMe(u8);

/// Because this type contains a blocklisted type, it should not derive
/// Default. Instead, we should emit a `mem::zeroed` implementation.
#[repr(C)]
pub struct ShouldNotDeriveDefault {
    pub a: BlocklistMe,
}
#[test]
fn bindgen_test_layout_ShouldNotDeriveDefault() {
    const UNINIT: ::std::mem::MaybeUninit<ShouldNotDeriveDefault> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ShouldNotDeriveDefault>(),
        1usize,
        concat!("Size of: ", stringify!(ShouldNotDeriveDefault))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldNotDeriveDefault>(),
        1usize,
        concat!("Alignment of ", stringify!(ShouldNotDeriveDefault))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldNotDeriveDefault),
            "::",
            stringify!(a)
        )
    );
}
impl Default for ShouldNotDeriveDefault {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
