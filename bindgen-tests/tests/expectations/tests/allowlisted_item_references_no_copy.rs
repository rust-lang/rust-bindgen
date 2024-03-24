#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default)]
pub struct NoCopy {
    pub _address: u8,
}
const _: () = {
    assert!(::std::mem::size_of::<NoCopy>() == 1usize, "Size of NoCopy");
    assert!(::std::mem::align_of::<NoCopy>() == 1usize, "Alignment of NoCopy");
};
#[repr(C)]
#[derive(Debug, Default)]
pub struct AllowlistMe {
    pub a: NoCopy,
}
const _: () = {
    assert!(::std::mem::size_of::<AllowlistMe>() == 1usize, "Size of AllowlistMe");
    assert!(::std::mem::align_of::<AllowlistMe>() == 1usize, "Alignment of AllowlistMe");
    assert!(
        ::std::mem::offset_of!(AllowlistMe, a) == 0usize,
        "Offset of field: AllowlistMe::a",
    );
};
