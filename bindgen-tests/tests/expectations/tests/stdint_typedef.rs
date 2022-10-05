#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern "C" {
    pub fn fun() -> u64;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Struct {
    pub field: u64,
}
#[test]
fn bindgen_test_layout_Struct() {
    const UNINIT: ::std::mem::MaybeUninit<Struct> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Struct>(),
        8usize,
        concat!("Size of: ", stringify!(Struct))
    );
    assert_eq!(
        ::std::mem::align_of::<Struct>(),
        8usize,
        concat!("Alignment of ", stringify!(Struct))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).field) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Struct),
            "::",
            stringify!(field)
        )
    );
}
