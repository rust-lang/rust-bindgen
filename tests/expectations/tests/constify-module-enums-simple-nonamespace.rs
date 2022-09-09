#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub mod one_Foo {
    pub type Type = ::std::os::raw::c_int;
    pub const Variant1: Type = 0;
    pub const Variant2: Type = 1;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Bar {
    pub baz1: one_Foo::Type,
    pub baz2: *mut one_Foo::Type,
}
#[test]
fn bindgen_test_layout_Bar() {
    const UNINIT: ::std::mem::MaybeUninit<Bar> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Bar>(),
        16usize,
        concat!("Size of: ", stringify!(Bar))
    );
    assert_eq!(
        ::std::mem::align_of::<Bar>(),
        8usize,
        concat!("Alignment of ", stringify!(Bar))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).baz1) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Bar), "::", stringify!(baz1))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).baz2) as usize - ptr as usize },
        8usize,
        concat!("Offset of field: ", stringify!(Bar), "::", stringify!(baz2))
    );
}
impl Default for Bar {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
