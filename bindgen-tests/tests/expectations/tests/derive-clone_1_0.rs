#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/** Since builtin `Clone` impls were introduced in Rust 1.21 this struct
 should impl `Clone` "manually".*/
#[repr(C)]
#[derive(Copy)]
pub struct ShouldImplClone {
    pub large: [::std::os::raw::c_int; 33usize],
}
#[test]
fn bindgen_test_layout_ShouldImplClone() {
    const UNINIT: ::std::mem::MaybeUninit<ShouldImplClone> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ShouldImplClone>(),
        132usize,
        "Size of ShouldImplClone",
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldImplClone>(),
        4usize,
        "Alignment of ShouldImplClone",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).large) as usize - ptr as usize },
        0usize,
        "Offset of field: ShouldImplClone::large",
    );
}
impl Clone for ShouldImplClone {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ShouldImplClone {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
