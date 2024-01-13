#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rooted {
    pub ptr: MaybeWrapped<::std::os::raw::c_int>,
}
#[test]
fn bindgen_test_layout_Rooted() {
    const UNINIT: ::std::mem::MaybeUninit<Rooted> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Rooted>(),
        4usize,
        concat!("Size of: ", stringify!(Rooted)),
    );
    assert_eq!(
        ::std::mem::align_of::<Rooted>(),
        4usize,
        concat!("Alignment of ", stringify!(Rooted)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ptr) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Rooted), "::", stringify!(ptr)),
    );
}
impl Default for Rooted {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// <div rustbindgen replaces="MaybeWrapped"></div>
pub type MaybeWrapped<a> = a;
#[test]
fn __bindgen_test_layout_MaybeWrapped_open0_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<MaybeWrapped<::std::os::raw::c_int>>(),
        4usize,
        concat!(
            "Size of template specialization: ",
            stringify!(MaybeWrapped < ::std::os::raw::c_int >),
        ),
    );
    assert_eq!(
        ::std::mem::align_of::<MaybeWrapped<::std::os::raw::c_int>>(),
        4usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(MaybeWrapped < ::std::os::raw::c_int >),
        ),
    );
}
