#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Bar {
    pub m_baz: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Bar() {
    const UNINIT: ::std::mem::MaybeUninit<Bar> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Bar>(),
        4usize,
        concat!("Size of: ", stringify!(Bar))
    );
    assert_eq!(
        ::std::mem::align_of::<Bar>(),
        4usize,
        concat!("Alignment of ", stringify!(Bar))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m_baz) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Bar),
            "::",
            stringify!(m_baz)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN3BarC1Ei"]
    pub fn Bar_Bar(this: *mut Bar, baz: ::std::os::raw::c_int);
}
impl Bar {
    #[inline]
    pub unsafe fn new(baz: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        Bar_Bar(__bindgen_tmp.as_mut_ptr(), baz);
        __bindgen_tmp.assume_init()
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Baz {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}_ZN3Baz3FOOE"]
    pub static Baz_FOO: [Bar; 0usize];
}
#[test]
fn bindgen_test_layout_Baz() {
    assert_eq!(
        ::std::mem::size_of::<Baz>(),
        1usize,
        concat!("Size of: ", stringify!(Baz))
    );
    assert_eq!(
        ::std::mem::align_of::<Baz>(),
        1usize,
        concat!("Alignment of ", stringify!(Baz))
    );
}
