#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct mozilla_FragmentOrURL {
    pub mIsLocalRef: bool,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of mozilla_FragmentOrURL",
    ][::std::mem::size_of::<mozilla_FragmentOrURL>() - 1usize];
    [
        "Alignment of mozilla_FragmentOrURL",
    ][::std::mem::align_of::<mozilla_FragmentOrURL>() - 1usize];
    [
        "Offset of field: mozilla_FragmentOrURL::mIsLocalRef",
    ][::std::mem::offset_of!(mozilla_FragmentOrURL, mIsLocalRef) - 0usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct mozilla_Position {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of mozilla_Position"][::std::mem::size_of::<mozilla_Position>() - 1usize];
    [
        "Alignment of mozilla_Position",
    ][::std::mem::align_of::<mozilla_Position>() - 1usize];
};
#[repr(C)]
pub struct mozilla_StyleShapeSource {
    pub __bindgen_anon_1: mozilla_StyleShapeSource__bindgen_ty_1,
}
#[repr(C)]
pub union mozilla_StyleShapeSource__bindgen_ty_1 {
    pub mPosition: *mut mozilla_Position,
    pub mFragmentOrURL: *mut mozilla_FragmentOrURL,
}
impl Default for mozilla_StyleShapeSource__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for mozilla_StyleShapeSource {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Bar {
    pub mFoo: *mut nsFoo,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Bar"][::std::mem::size_of::<Bar>() - 8usize];
    ["Alignment of Bar"][::std::mem::align_of::<Bar>() - 8usize];
    ["Offset of field: Bar::mFoo"][::std::mem::offset_of!(Bar, mFoo) - 0usize];
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
pub struct nsFoo {
    pub mBar: mozilla_StyleShapeSource,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of nsFoo"][::std::mem::size_of::<nsFoo>() - 8usize];
    ["Alignment of nsFoo"][::std::mem::align_of::<nsFoo>() - 8usize];
    ["Offset of field: nsFoo::mBar"][::std::mem::offset_of!(nsFoo, mBar) - 0usize];
};
impl Default for nsFoo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: mozilla_StyleShapeSource_open0_int_close0",
    ][::std::mem::size_of::<mozilla_StyleShapeSource>() - 8usize];
    [
        "Align of template specialization: mozilla_StyleShapeSource_open0_int_close0",
    ][::std::mem::align_of::<mozilla_StyleShapeSource>() - 8usize];
};
