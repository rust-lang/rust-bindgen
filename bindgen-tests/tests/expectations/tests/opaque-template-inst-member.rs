#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OpaqueTemplate {
    pub _address: u8,
}
/// This should not end up deriving Debug/Hash because its `mBlah` field cannot derive
/// Debug/Hash because the instantiation's definition cannot derive Debug/Hash.
#[repr(C)]
pub struct ContainsOpaqueTemplate {
    pub mBlah: [u32; 101usize],
    pub mBaz: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ContainsOpaqueTemplate() {
    const UNINIT: ::std::mem::MaybeUninit<ContainsOpaqueTemplate> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ContainsOpaqueTemplate>(),
        408usize,
        "Size of ContainsOpaqueTemplate",
    );
    assert_eq!(
        ::std::mem::align_of::<ContainsOpaqueTemplate>(),
        4usize,
        "Alignment of ContainsOpaqueTemplate",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mBlah) as usize - ptr as usize },
        0usize,
        "Offset of field: ContainsOpaqueTemplate::mBlah",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mBaz) as usize - ptr as usize },
        404usize,
        "Offset of field: ContainsOpaqueTemplate::mBaz",
    );
}
impl Default for ContainsOpaqueTemplate {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::cmp::PartialEq for ContainsOpaqueTemplate {
    fn eq(&self, other: &ContainsOpaqueTemplate) -> bool {
        &self.mBlah[..] == &other.mBlah[..] && self.mBaz == other.mBaz
    }
}
/// This should not end up deriving Debug/Hash either, for similar reasons, although
/// we're exercising base member edges now.
#[repr(C)]
pub struct InheritsOpaqueTemplate {
    pub _base: [u8; 401usize],
    pub wow: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_InheritsOpaqueTemplate() {
    const UNINIT: ::std::mem::MaybeUninit<InheritsOpaqueTemplate> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<InheritsOpaqueTemplate>(),
        416usize,
        "Size of InheritsOpaqueTemplate",
    );
    assert_eq!(
        ::std::mem::align_of::<InheritsOpaqueTemplate>(),
        8usize,
        "Alignment of InheritsOpaqueTemplate",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).wow) as usize - ptr as usize },
        408usize,
        "Offset of field: InheritsOpaqueTemplate::wow",
    );
}
impl Default for InheritsOpaqueTemplate {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::cmp::PartialEq for InheritsOpaqueTemplate {
    fn eq(&self, other: &InheritsOpaqueTemplate) -> bool {
        &self._base[..] == &other._base[..] && self.wow == other.wow
    }
}
