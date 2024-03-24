#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Bar {
    pub b: *mut a,
}
const _: () = {
    ["Size of Bar"][::std::mem::size_of::<Bar>() - 8usize];
    ["Alignment of Bar"][::std::mem::align_of::<Bar>() - 8usize];
    ["Offset of field: Bar::b"][::std::mem::offset_of!(Bar, b) - 0usize];
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
    ["Size of c__bindgen_ty_1"][::std::mem::size_of::<c__bindgen_ty_1>() - 1usize];
    ["Alignment of c__bindgen_ty_1"][::std::mem::align_of::<c__bindgen_ty_1>() - 1usize];
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
    ["Size of c"][::std::mem::size_of::<c>() - 1usize];
    ["Alignment of c"][::std::mem::align_of::<c>() - 1usize];
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
    ["Size of a"][::std::mem::size_of::<a>() - 1usize];
    ["Alignment of a"][::std::mem::align_of::<a>() - 1usize];
    ["Offset of field: a::d"][::std::mem::offset_of!(a, d) - 0usize];
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
