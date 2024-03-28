#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s {
    pub u: s__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union s__bindgen_ty_1 {
    pub field: s__bindgen_ty_1_inner,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct s__bindgen_ty_1_inner {
    pub b: ::std::os::raw::c_int,
}
const _: () = {
    assert!(
        ::std::mem::size_of::<s__bindgen_ty_1_inner>() == 4usize,
        "Size of s__bindgen_ty_1_inner",
    );
    assert!(
        ::std::mem::align_of::<s__bindgen_ty_1_inner>() == 4usize,
        "Alignment of s__bindgen_ty_1_inner",
    );
    assert!(
        ::std::mem::offset_of!(s__bindgen_ty_1_inner, b) == 0usize,
        "Offset of field: s__bindgen_ty_1_inner::b",
    );
};
const _: () = {
    assert!(
        ::std::mem::size_of::<s__bindgen_ty_1>() == 4usize,
        "Size of s__bindgen_ty_1",
    );
    assert!(
        ::std::mem::align_of::<s__bindgen_ty_1>() == 4usize,
        "Alignment of s__bindgen_ty_1",
    );
    assert!(
        ::std::mem::offset_of!(s__bindgen_ty_1, field) == 0usize,
        "Offset of field: s__bindgen_ty_1::field",
    );
};
impl Default for s__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
const _: () = {
    assert!(::std::mem::size_of::<s>() == 4usize, "Size of s");
    assert!(::std::mem::align_of::<s>() == 4usize, "Alignment of s");
    assert!(::std::mem::offset_of!(s, u) == 0usize, "Offset of field: s::u");
};
impl Default for s {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
