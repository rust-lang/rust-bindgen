#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// We emit a `[u8; 63usize]` padding field for this struct, which cannot derive
/// Debug/Hash because 63 is over the hard coded limit.
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct NoDebug {
    pub c: ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_NoDebug() {
    const UNINIT: ::std::mem::MaybeUninit<NoDebug> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<NoDebug>(), 64usize, "Size of NoDebug");
    assert_eq!(::std::mem::align_of::<NoDebug>(), 64usize, "Alignment of NoDebug");
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).c) as usize - ptr as usize },
        0usize,
        "Offset of field: NoDebug::c",
    );
}
impl Default for NoDebug {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::cmp::PartialEq for NoDebug {
    fn eq(&self, other: &NoDebug) -> bool {
        self.c == other.c
    }
}
/// This should derive Debug/Hash/PartialEq/Eq because the padding size is less than the max derive
/// Debug/Hash/PartialEq/Eq impl for arrays. However, we conservatively don't derive Debug/Hash because
/// we determine Debug derive-ability before we compute padding, which happens at
/// codegen.
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct ShouldDeriveDebugButDoesNot {
    pub c: [::std::os::raw::c_char; 32usize],
    pub d: ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_ShouldDeriveDebugButDoesNot() {
    const UNINIT: ::std::mem::MaybeUninit<ShouldDeriveDebugButDoesNot> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ShouldDeriveDebugButDoesNot>(),
        64usize,
        "Size of ShouldDeriveDebugButDoesNot",
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldDeriveDebugButDoesNot>(),
        64usize,
        "Alignment of ShouldDeriveDebugButDoesNot",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).c) as usize - ptr as usize },
        0usize,
        "Offset of field: ShouldDeriveDebugButDoesNot::c",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).d) as usize - ptr as usize },
        32usize,
        "Offset of field: ShouldDeriveDebugButDoesNot::d",
    );
}
impl Default for ShouldDeriveDebugButDoesNot {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::cmp::PartialEq for ShouldDeriveDebugButDoesNot {
    fn eq(&self, other: &ShouldDeriveDebugButDoesNot) -> bool {
        self.c == other.c && self.d == other.d
    }
}
