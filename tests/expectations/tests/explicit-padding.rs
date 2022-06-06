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
    fn test_field_first() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pad_me>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).first) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pad_me),
                "::",
                stringify!(first)
            )
        );
    }
    test_field_first();
    fn test_field_second() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pad_me>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).second) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(pad_me),
                "::",
                stringify!(second)
            )
        );
    }
    test_field_second();
    fn test_field_third() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pad_me>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).third) as usize - ptr as usize
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
    test_field_third();
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
    fn test_field_first() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<dont_pad_me>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).first) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(dont_pad_me),
                "::",
                stringify!(first)
            )
        );
    }
    test_field_first();
    fn test_field_second() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<dont_pad_me>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).second) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(dont_pad_me),
                "::",
                stringify!(second)
            )
        );
    }
    test_field_second();
    fn test_field_third() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<dont_pad_me>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).third) as usize - ptr as usize
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
    test_field_third();
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
