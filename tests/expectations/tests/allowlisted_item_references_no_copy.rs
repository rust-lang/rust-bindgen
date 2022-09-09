#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default)]
pub struct NoCopy {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_NoCopy() {
    assert_eq!(
        ::std::mem::size_of::<NoCopy>(),
        1usize,
        concat!("Size of: ", stringify!(NoCopy))
    );
    assert_eq!(
        ::std::mem::align_of::<NoCopy>(),
        1usize,
        concat!("Alignment of ", stringify!(NoCopy))
    );
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct AllowlistMe {
    pub a: NoCopy,
}
#[test]
fn bindgen_test_layout_AllowlistMe() {
    const UNINIT: ::std::mem::MaybeUninit<AllowlistMe> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AllowlistMe>(),
        1usize,
        concat!("Size of: ", stringify!(AllowlistMe))
    );
    assert_eq!(
        ::std::mem::align_of::<AllowlistMe>(),
        1usize,
        concat!("Alignment of ", stringify!(AllowlistMe))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AllowlistMe),
            "::",
            stringify!(a)
        )
    );
}
