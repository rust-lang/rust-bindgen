#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(transparent)]
pub struct __BindgenFloat16(pub u16);
extern "C" {
    pub static mut global: __BindgenFloat16;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Test__Float16 {
    pub f: __BindgenFloat16,
}
#[test]
fn bindgen_test_layout_Test__Float16() {
    const UNINIT: ::std::mem::MaybeUninit<Test__Float16> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Test__Float16>(),
        2usize,
        concat!("Size of: ", stringify!(Test__Float16)),
    );
    assert_eq!(
        ::std::mem::align_of::<Test__Float16>(),
        2usize,
        concat!("Alignment of ", stringify!(Test__Float16)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).f) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Test__Float16), "::", stringify!(f)),
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Test__Float16Ref {
    pub f: *mut __BindgenFloat16,
}
#[test]
fn bindgen_test_layout_Test__Float16Ref() {
    const UNINIT: ::std::mem::MaybeUninit<Test__Float16Ref> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Test__Float16Ref>(),
        8usize,
        concat!("Size of: ", stringify!(Test__Float16Ref)),
    );
    assert_eq!(
        ::std::mem::align_of::<Test__Float16Ref>(),
        8usize,
        concat!("Alignment of ", stringify!(Test__Float16Ref)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).f) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Test__Float16Ref), "::", stringify!(f)),
    );
}
impl Default for Test__Float16Ref {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
