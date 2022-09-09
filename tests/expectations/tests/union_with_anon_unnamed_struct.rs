#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone)]
pub union pixel {
    pub rgba: ::std::os::raw::c_uint,
    pub __bindgen_anon_1: pixel__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct pixel__bindgen_ty_1 {
    pub r: ::std::os::raw::c_uchar,
    pub g: ::std::os::raw::c_uchar,
    pub b: ::std::os::raw::c_uchar,
    pub a: ::std::os::raw::c_uchar,
}
#[test]
fn bindgen_test_layout_pixel__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<pixel__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<pixel__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(pixel__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<pixel__bindgen_ty_1>(),
        1usize,
        concat!("Alignment of ", stringify!(pixel__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pixel__bindgen_ty_1),
            "::",
            stringify!(r)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).g) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(pixel__bindgen_ty_1),
            "::",
            stringify!(g)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(pixel__bindgen_ty_1),
            "::",
            stringify!(b)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(pixel__bindgen_ty_1),
            "::",
            stringify!(a)
        )
    );
}
#[test]
fn bindgen_test_layout_pixel() {
    const UNINIT: ::std::mem::MaybeUninit<pixel> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<pixel>(),
        4usize,
        concat!("Size of: ", stringify!(pixel))
    );
    assert_eq!(
        ::std::mem::align_of::<pixel>(),
        4usize,
        concat!("Alignment of ", stringify!(pixel))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rgba) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pixel),
            "::",
            stringify!(rgba)
        )
    );
}
impl Default for pixel {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
