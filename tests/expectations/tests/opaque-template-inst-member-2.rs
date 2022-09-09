#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// This is like `opaque-template-inst-member.hpp` except exercising the cases
/// where we are OK to derive Debug/Hash/PartialEq.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OpaqueTemplate {
    pub _address: u8,
}
/// Should derive Debug/Hash/PartialEq.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ContainsOpaqueTemplate {
    pub mBlah: u32,
    pub mBaz: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ContainsOpaqueTemplate() {
    const UNINIT: ::std::mem::MaybeUninit<ContainsOpaqueTemplate> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ContainsOpaqueTemplate>(),
        8usize,
        concat!("Size of: ", stringify!(ContainsOpaqueTemplate))
    );
    assert_eq!(
        ::std::mem::align_of::<ContainsOpaqueTemplate>(),
        4usize,
        concat!("Alignment of ", stringify!(ContainsOpaqueTemplate))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mBlah) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ContainsOpaqueTemplate),
            "::",
            stringify!(mBlah)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mBaz) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ContainsOpaqueTemplate),
            "::",
            stringify!(mBaz)
        )
    );
}
/// Should also derive Debug/Hash/PartialEq.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct InheritsOpaqueTemplate {
    pub _base: u8,
    pub wow: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_InheritsOpaqueTemplate() {
    const UNINIT: ::std::mem::MaybeUninit<InheritsOpaqueTemplate> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<InheritsOpaqueTemplate>(),
        16usize,
        concat!("Size of: ", stringify!(InheritsOpaqueTemplate))
    );
    assert_eq!(
        ::std::mem::align_of::<InheritsOpaqueTemplate>(),
        8usize,
        concat!("Alignment of ", stringify!(InheritsOpaqueTemplate))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).wow) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(InheritsOpaqueTemplate),
            "::",
            stringify!(wow)
        )
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
