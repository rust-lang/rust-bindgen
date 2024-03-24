#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Bar {
    pub b: *mut a,
}
const _: () = {
    assert!(::std::mem::size_of::<Bar>() == 8usize, "Size of Bar");
    assert!(::std::mem::align_of::<Bar>() == 8usize, "Alignment of Bar");
    assert!(::std::mem::offset_of!(Bar, b) == 0usize, "Offset of field: Bar::b");
};
impl Default for Bar {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c {
    pub __bindgen_anon_1: c__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union c__bindgen_ty_1 {
    pub _address: u8,
}
const _: () = {
    assert!(
        ::std::mem::size_of::<c__bindgen_ty_1>() == 1usize,
        "Size of c__bindgen_ty_1",
    );
    assert!(
        ::std::mem::align_of::<c__bindgen_ty_1>() == 1usize,
        "Alignment of c__bindgen_ty_1",
    );
};
impl Default for c__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
const _: () = {
    assert!(::std::mem::size_of::<c>() == 1usize, "Size of c");
    assert!(::std::mem::align_of::<c>() == 1usize, "Alignment of c");
};
impl Default for c {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a {
    pub d: c,
}
const _: () = {
    assert!(::std::mem::size_of::<a>() == 1usize, "Size of a");
    assert!(::std::mem::align_of::<a>() == 1usize, "Alignment of a");
    assert!(::std::mem::offset_of!(a, d) == 0usize, "Offset of field: a::d");
};
impl Default for a {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
