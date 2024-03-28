#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Copy, Clone)]
pub union foo {
    pub a: ::std::os::raw::c_uint,
    pub __bindgen_anon_1: foo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union foo__bindgen_ty_1 {
    pub b: ::std::os::raw::c_ushort,
    pub c: ::std::os::raw::c_uchar,
}
const _: () = {
    assert!(
        ::std::mem::size_of::<foo__bindgen_ty_1>() == 2usize,
        "Size of foo__bindgen_ty_1",
    );
    assert!(
        ::std::mem::align_of::<foo__bindgen_ty_1>() == 2usize,
        "Alignment of foo__bindgen_ty_1",
    );
    assert!(
        ::std::mem::offset_of!(foo__bindgen_ty_1, b) == 0usize,
        "Offset of field: foo__bindgen_ty_1::b",
    );
    assert!(
        ::std::mem::offset_of!(foo__bindgen_ty_1, c) == 0usize,
        "Offset of field: foo__bindgen_ty_1::c",
    );
};
impl Default for foo__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
const _: () = {
    assert!(::std::mem::size_of::<foo>() == 4usize, "Size of foo");
    assert!(::std::mem::align_of::<foo>() == 4usize, "Alignment of foo");
    assert!(::std::mem::offset_of!(foo, a) == 0usize, "Offset of field: foo::a");
};
impl Default for foo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
