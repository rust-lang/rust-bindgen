#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
pub struct header {
    pub _bindgen_opaque_blob: [u8; 16usize],
}
#[test]
fn bindgen_test_layout_header() {
    assert_eq!(::std::mem::size_of::<header>(), 16usize, "Size of header");
}
impl Default for header {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
