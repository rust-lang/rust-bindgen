#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union Bar {
    pub foo: ::std::os::raw::c_uchar,
}
#[test]
fn bindgen_test_layout_Bar() {
    const UNINIT: ::std::mem::MaybeUninit<Bar> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<Bar>(), 16usize, "Size of Bar");
    assert_eq!(::std::mem::align_of::<Bar>(), 16usize, "Alignment of Bar");
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).foo) as usize - ptr as usize },
        0usize,
        "Offset of field: Bar::foo",
    );
}
impl Default for Bar {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union Baz {
    pub bar: Bar,
}
#[test]
fn bindgen_test_layout_Baz() {
    const UNINIT: ::std::mem::MaybeUninit<Baz> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<Baz>(), 16usize, "Size of Baz");
    assert_eq!(::std::mem::align_of::<Baz>(), 16usize, "Alignment of Baz");
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize },
        0usize,
        "Offset of field: Baz::bar",
    );
}
impl Default for Baz {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
