#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
const _: () = {
    assert!(
        ::std::mem::size_of::<pixel__bindgen_ty_1>() == 4usize,
        "Size of pixel__bindgen_ty_1",
    );
    assert!(
        ::std::mem::align_of::<pixel__bindgen_ty_1>() == 1usize,
        "Alignment of pixel__bindgen_ty_1",
    );
    assert!(
        ::std::mem::offset_of!(pixel__bindgen_ty_1, r) == 0usize,
        "Offset of field: pixel__bindgen_ty_1::r",
    );
    assert!(
        ::std::mem::offset_of!(pixel__bindgen_ty_1, g) == 1usize,
        "Offset of field: pixel__bindgen_ty_1::g",
    );
    assert!(
        ::std::mem::offset_of!(pixel__bindgen_ty_1, b) == 2usize,
        "Offset of field: pixel__bindgen_ty_1::b",
    );
    assert!(
        ::std::mem::offset_of!(pixel__bindgen_ty_1, a) == 3usize,
        "Offset of field: pixel__bindgen_ty_1::a",
    );
};
const _: () = {
    assert!(::std::mem::size_of::<pixel>() == 4usize, "Size of pixel");
    assert!(::std::mem::align_of::<pixel>() == 4usize, "Alignment of pixel");
    assert!(
        ::std::mem::offset_of!(pixel, rgba) == 0usize,
        "Offset of field: pixel::rgba",
    );
};
impl Default for pixel {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
