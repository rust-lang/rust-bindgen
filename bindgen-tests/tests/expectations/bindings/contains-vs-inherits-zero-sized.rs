#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// This should get an `_address` byte.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Empty {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Empty() {
    assert_eq!(
        ::std::mem::size_of::<Empty>(),
        1usize,
        concat!("Size of: ", stringify!(Empty))
    );
    assert_eq!(
        ::std::mem::align_of::<Empty>(),
        1usize,
        concat!("Alignment of ", stringify!(Empty))
    );
}
/// This should not get an `_address` byte, so `sizeof(Inherits)` should be
/// `1`.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Inherits {
    pub b: bool,
}
#[test]
fn bindgen_test_layout_Inherits() {
    const UNINIT: ::std::mem::MaybeUninit<Inherits> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Inherits>(),
        1usize,
        concat!("Size of: ", stringify!(Inherits))
    );
    assert_eq!(
        ::std::mem::align_of::<Inherits>(),
        1usize,
        concat!("Alignment of ", stringify!(Inherits))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Inherits),
            "::",
            stringify!(b)
        )
    );
}
/// This should not get an `_address` byte, but contains `Empty` which *does* get
/// one, so `sizeof(Contains)` should be `1 + 1`.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Contains {
    pub empty: Empty,
    pub b: bool,
}
#[test]
fn bindgen_test_layout_Contains() {
    const UNINIT: ::std::mem::MaybeUninit<Contains> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Contains>(),
        2usize,
        concat!("Size of: ", stringify!(Contains))
    );
    assert_eq!(
        ::std::mem::align_of::<Contains>(),
        1usize,
        concat!("Alignment of ", stringify!(Contains))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).empty) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Contains),
            "::",
            stringify!(empty)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(Contains),
            "::",
            stringify!(b)
        )
    );
}
