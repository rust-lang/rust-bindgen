#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct pad_me {
    pub first: u8,
    pub __bindgen_padding_0: [u8; 3usize],
    pub second: u32,
    pub third: u16,
    pub __bindgen_padding_1: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_pad_me() {
    assert_eq!(
        ::std::mem::size_of::<pad_me>(),
        12usize,
        concat!("Size of: ", stringify!(pad_me))
    );
    assert_eq!(
        ::std::mem::align_of::<pad_me>(),
        4usize,
        concat!("Alignment of ", stringify!(pad_me))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<pad_me>())).first as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pad_me),
            "::",
            stringify!(first)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<pad_me>())).second as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(pad_me),
            "::",
            stringify!(second)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<pad_me>())).third as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(pad_me),
            "::",
            stringify!(third)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union dont_pad_me {
    pub first: u8,
    pub second: u32,
    pub third: u16,
}
#[test]
fn bindgen_test_layout_dont_pad_me() {
    assert_eq!(
        ::std::mem::size_of::<dont_pad_me>(),
        4usize,
        concat!("Size of: ", stringify!(dont_pad_me))
    );
    assert_eq!(
        ::std::mem::align_of::<dont_pad_me>(),
        4usize,
        concat!("Alignment of ", stringify!(dont_pad_me))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<dont_pad_me>())).first as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(dont_pad_me),
            "::",
            stringify!(first)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<dont_pad_me>())).second as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(dont_pad_me),
            "::",
            stringify!(second)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<dont_pad_me>())).third as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(dont_pad_me),
            "::",
            stringify!(third)
        )
    );
}
impl Default for dont_pad_me {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
