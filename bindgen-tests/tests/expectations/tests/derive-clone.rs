#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// This struct should derive `Clone`.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShouldDeriveClone {
    pub large: [::std::os::raw::c_int; 33usize],
}
#[test]
fn bindgen_test_layout_ShouldDeriveClone() {
    const UNINIT: ::std::mem::MaybeUninit<ShouldDeriveClone> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ShouldDeriveClone>(),
        132usize,
        "Size of ShouldDeriveClone",
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldDeriveClone>(),
        4usize,
        "Alignment of ShouldDeriveClone",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).large) as usize - ptr as usize },
        0usize,
        "Offset of field: ShouldDeriveClone::large",
    );
}
impl Default for ShouldDeriveClone {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
