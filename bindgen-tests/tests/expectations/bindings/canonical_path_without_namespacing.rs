#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Bar {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(
        ::std::mem::size_of::<Bar>(),
        1usize,
        concat!("Size of: ", stringify!(Bar))
    );
    assert_eq!(
        ::std::mem::align_of::<Bar>(),
        1usize,
        concat!("Alignment of ", stringify!(Bar))
    );
}
extern "C" {
    #[link_name = "\u{1}_Z3bazPN3foo3BarE"]
    pub fn baz(arg1: *mut Bar);
}
