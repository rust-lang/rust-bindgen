#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Base {
    pub large: [::std::os::raw::c_int; 33usize],
}
#[test]
fn bindgen_test_layout_Base() {
    const UNINIT: ::std::mem::MaybeUninit<Base> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<Base>(), 132usize, "Size of Base");
    assert_eq!(::std::mem::align_of::<Base>(), 4usize, "Alignment of Base");
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).large) as usize - ptr as usize },
        0usize,
        "Offset of field: Base::large",
    );
}
impl Default for Base {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::cmp::PartialEq for Base {
    fn eq(&self, other: &Base) -> bool {
        &self.large[..] == &other.large[..]
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShouldDerivePartialEq {
    pub _base: Base,
}
#[test]
fn bindgen_test_layout_ShouldDerivePartialEq() {
    assert_eq!(
        ::std::mem::size_of::<ShouldDerivePartialEq>(),
        132usize,
        "Size of ShouldDerivePartialEq",
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldDerivePartialEq>(),
        4usize,
        "Alignment of ShouldDerivePartialEq",
    );
}
impl Default for ShouldDerivePartialEq {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::cmp::PartialEq for ShouldDerivePartialEq {
    fn eq(&self, other: &ShouldDerivePartialEq) -> bool {
        self._base == other._base
    }
}
