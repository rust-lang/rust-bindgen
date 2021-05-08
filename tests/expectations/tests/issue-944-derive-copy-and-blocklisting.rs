#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub struct BlocklistMe(u8);

/// Because this type contains a blocklisted type, it should not derive Copy.
#[repr(C)]
pub struct ShouldNotBeCopy {
    pub a: BlocklistMe,
}
#[test]
fn bindgen_test_layout_ShouldNotBeCopy() {
    assert_eq!(
        ::std::mem::size_of::<ShouldNotBeCopy>(),
        1usize,
        concat!("Size of: ", stringify!(ShouldNotBeCopy))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldNotBeCopy>(),
        1usize,
        concat!("Alignment of ", stringify!(ShouldNotBeCopy))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ShouldNotBeCopy>())).a as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldNotBeCopy),
            "::",
            stringify!(a)
        )
    );
}
impl Default for ShouldNotBeCopy {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
