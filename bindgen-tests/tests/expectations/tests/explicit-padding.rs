#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct pad_me {
    pub first: u8,
    pub __bindgen_padding_0: [u8; 3usize],
    pub second: u32,
    pub third: u16,
    pub __bindgen_padding_1: [u8; 2usize],
}
const _: () = {
    assert!(::std::mem::size_of::<pad_me>() == 12usize, "Size of pad_me");
    assert!(::std::mem::align_of::<pad_me>() == 4usize, "Alignment of pad_me");
    assert!(
        ::std::mem::offset_of!(pad_me, first) == 0usize,
        "Offset of field: pad_me::first",
    );
    assert!(
        ::std::mem::offset_of!(pad_me, second) == 4usize,
        "Offset of field: pad_me::second",
    );
    assert!(
        ::std::mem::offset_of!(pad_me, third) == 8usize,
        "Offset of field: pad_me::third",
    );
};
#[repr(C)]
#[derive(Copy, Clone)]
pub union dont_pad_me {
    pub first: u8,
    pub second: u32,
    pub third: u16,
}
const _: () = {
    assert!(::std::mem::size_of::<dont_pad_me>() == 4usize, "Size of dont_pad_me");
    assert!(::std::mem::align_of::<dont_pad_me>() == 4usize, "Alignment of dont_pad_me");
    assert!(
        ::std::mem::offset_of!(dont_pad_me, first) == 0usize,
        "Offset of field: dont_pad_me::first",
    );
    assert!(
        ::std::mem::offset_of!(dont_pad_me, second) == 0usize,
        "Offset of field: dont_pad_me::second",
    );
    assert!(
        ::std::mem::offset_of!(dont_pad_me, third) == 0usize,
        "Offset of field: dont_pad_me::third",
    );
};
impl Default for dont_pad_me {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
