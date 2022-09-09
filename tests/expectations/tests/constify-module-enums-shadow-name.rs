#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub mod foo {
    pub type Type = ::std::os::raw::c_uint;
    pub const Type: Type = 0;
    pub const Type_: Type = 1;
    pub const Type1: Type = 2;
    pub const Type__: Type = 3;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bar {
    pub member: foo::Type,
}
#[test]
fn bindgen_test_layout_bar() {
    const UNINIT: ::std::mem::MaybeUninit<bar> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<bar>(),
        4usize,
        concat!("Size of: ", stringify!(bar))
    );
    assert_eq!(
        ::std::mem::align_of::<bar>(),
        4usize,
        concat!("Alignment of ", stringify!(bar))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).member) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bar),
            "::",
            stringify!(member)
        )
    );
}
impl Default for bar {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
