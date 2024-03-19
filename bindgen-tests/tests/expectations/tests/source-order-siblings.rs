#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const ROOT: &[u8; 5] = b"root\0";
pub const A: ::std::os::raw::c_char = 97;
extern "C" {
    pub fn AA();
}
pub const B: ::std::os::raw::c_char = 'b' as ::std::os::raw::c_char;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct BB {}
#[test]
fn bindgen_test_layout_BB() {
    assert_eq!(
        ::std::mem::size_of::<BB>(),
        0usize,
        concat!("Size of: ", stringify!(BB)),
    );
    assert_eq!(
        ::std::mem::align_of::<BB>(),
        1usize,
        concat!("Alignment of ", stringify!(BB)),
    );
}
