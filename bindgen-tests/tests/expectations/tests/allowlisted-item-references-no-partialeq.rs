#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct NoPartialEq {
    pub _address: u8,
}
const _: () = {
    assert!(::std::mem::size_of::<NoPartialEq>() == 1usize, "Size of NoPartialEq");
    assert!(::std::mem::align_of::<NoPartialEq>() == 1usize, "Alignment of NoPartialEq");
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AllowlistMe {
    pub a: NoPartialEq,
}
const _: () = {
    assert!(::std::mem::size_of::<AllowlistMe>() == 1usize, "Size of AllowlistMe");
    assert!(::std::mem::align_of::<AllowlistMe>() == 1usize, "Alignment of AllowlistMe");
    assert!(
        ::std::mem::offset_of!(AllowlistMe, a) == 0usize,
        "Offset of field: AllowlistMe::a",
    );
};
