#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsFoo {
    pub details: [f32; 400usize],
}
#[test]
fn bindgen_test_layout_nsFoo() {
    const UNINIT: ::std::mem::MaybeUninit<nsFoo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<nsFoo>(), 1600usize, "Size of nsFoo");
    assert_eq!(::std::mem::align_of::<nsFoo>(), 4usize, "Alignment of nsFoo");
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).details) as usize - ptr as usize },
        0usize,
        "Offset of field: nsFoo::details",
    );
}
impl Default for nsFoo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub static gDetails: nsFoo;
}
