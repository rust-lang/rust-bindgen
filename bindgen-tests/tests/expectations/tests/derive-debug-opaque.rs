#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[repr(align(4))]
pub struct Opaque {
    pub _bindgen_opaque_blob: [u32; 41usize],
}
#[test]
fn bindgen_test_layout_Opaque() {
    assert_eq!(::std::mem::size_of::<Opaque>(), 164usize, "Size of Opaque");
    assert_eq!(::std::mem::align_of::<Opaque>(), 4usize, "Alignment of Opaque");
}
impl Default for Opaque {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for Opaque {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "Opaque {{ opaque }}")
    }
}
#[repr(C)]
pub struct OpaqueUser {
    pub opaque: Opaque,
}
#[test]
fn bindgen_test_layout_OpaqueUser() {
    const UNINIT: ::std::mem::MaybeUninit<OpaqueUser> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<OpaqueUser>(), 164usize, "Size of OpaqueUser");
    assert_eq!(::std::mem::align_of::<OpaqueUser>(), 4usize, "Alignment of OpaqueUser");
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).opaque) as usize - ptr as usize },
        0usize,
        "Offset of field: OpaqueUser::opaque",
    );
}
impl Default for OpaqueUser {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for OpaqueUser {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "OpaqueUser {{ opaque: {:?} }}", self.opaque)
    }
}
