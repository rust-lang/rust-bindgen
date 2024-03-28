#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Copy, Clone)]
pub union color {
    pub u1: color__bindgen_ty_1,
    pub u2: color__bindgen_ty_2,
    pub v3: [::std::os::raw::c_uchar; 3usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct color__bindgen_ty_1 {
    pub r: ::std::os::raw::c_uchar,
    pub g: ::std::os::raw::c_uchar,
    pub b: ::std::os::raw::c_uchar,
}
const _: () = {
    assert!(
        ::std::mem::size_of::<color__bindgen_ty_1>() == 3usize,
        "Size of color__bindgen_ty_1",
    );
    assert!(
        ::std::mem::align_of::<color__bindgen_ty_1>() == 1usize,
        "Alignment of color__bindgen_ty_1",
    );
    assert!(
        ::std::mem::offset_of!(color__bindgen_ty_1, r) == 0usize,
        "Offset of field: color__bindgen_ty_1::r",
    );
    assert!(
        ::std::mem::offset_of!(color__bindgen_ty_1, g) == 1usize,
        "Offset of field: color__bindgen_ty_1::g",
    );
    assert!(
        ::std::mem::offset_of!(color__bindgen_ty_1, b) == 2usize,
        "Offset of field: color__bindgen_ty_1::b",
    );
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct color__bindgen_ty_2 {
    pub y: ::std::os::raw::c_uchar,
    pub u: ::std::os::raw::c_uchar,
    pub v: ::std::os::raw::c_uchar,
}
const _: () = {
    assert!(
        ::std::mem::size_of::<color__bindgen_ty_2>() == 3usize,
        "Size of color__bindgen_ty_2",
    );
    assert!(
        ::std::mem::align_of::<color__bindgen_ty_2>() == 1usize,
        "Alignment of color__bindgen_ty_2",
    );
    assert!(
        ::std::mem::offset_of!(color__bindgen_ty_2, y) == 0usize,
        "Offset of field: color__bindgen_ty_2::y",
    );
    assert!(
        ::std::mem::offset_of!(color__bindgen_ty_2, u) == 1usize,
        "Offset of field: color__bindgen_ty_2::u",
    );
    assert!(
        ::std::mem::offset_of!(color__bindgen_ty_2, v) == 2usize,
        "Offset of field: color__bindgen_ty_2::v",
    );
};
const _: () = {
    assert!(::std::mem::size_of::<color>() == 3usize, "Size of color");
    assert!(::std::mem::align_of::<color>() == 1usize, "Alignment of color");
    assert!(::std::mem::offset_of!(color, v3) == 0usize, "Offset of field: color::v3");
};
impl Default for color {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
