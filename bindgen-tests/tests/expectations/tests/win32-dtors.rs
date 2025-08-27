#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default)]
pub struct CppObj {
    pub x: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of CppObj"][::std::mem::size_of::<CppObj>() - 4usize];
    ["Alignment of CppObj"][::std::mem::align_of::<CppObj>() - 4usize];
    ["Offset of field: CppObj::x"][::std::mem::offset_of!(CppObj, x) - 0usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}??0CppObj@@QEAA@H@Z"]
    pub fn CppObj_CppObj(this: *mut CppObj, x: ::std::os::raw::c_int);
}
unsafe extern "C" {
    #[link_name = "\u{1}??1CppObj@@QEAA@XZ"]
    pub fn CppObj_CppObj_destructor(this: *mut CppObj);
}
impl CppObj {
    #[inline]
    pub unsafe fn new(x: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        CppObj_CppObj(__bindgen_tmp.as_mut_ptr(), x);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        CppObj_CppObj_destructor(self)
    }
}
#[repr(C)]
pub struct CppObj2__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug)]
pub struct CppObj2 {
    pub vtable_: *const CppObj2__bindgen_vtable,
    pub x: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of CppObj2"][::std::mem::size_of::<CppObj2>() - 16usize];
    ["Alignment of CppObj2"][::std::mem::align_of::<CppObj2>() - 8usize];
    ["Offset of field: CppObj2::x"][::std::mem::offset_of!(CppObj2, x) - 8usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}??0CppObj2@@QEAA@H@Z"]
    pub fn CppObj2_CppObj2(this: *mut CppObj2, x: ::std::os::raw::c_int);
}
impl Default for CppObj2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl CppObj2 {
    #[inline]
    pub unsafe fn new(x: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        CppObj2_CppObj2(__bindgen_tmp.as_mut_ptr(), x);
        __bindgen_tmp.assume_init()
    }
}
unsafe extern "C" {
    #[link_name = "\u{1}??1CppObj2@@UEAA@XZ"]
    pub fn CppObj2_CppObj2_destructor(this: *mut CppObj2);
}
#[repr(C)]
#[derive(Debug)]
pub struct CppObj3 {
    pub _base: CppObj2,
    pub x: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of CppObj3"][::std::mem::size_of::<CppObj3>() - 24usize];
    ["Alignment of CppObj3"][::std::mem::align_of::<CppObj3>() - 8usize];
    ["Offset of field: CppObj3::x"][::std::mem::offset_of!(CppObj3, x) - 16usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}??0CppObj3@@QEAA@H@Z"]
    pub fn CppObj3_CppObj3(this: *mut CppObj3, x: ::std::os::raw::c_int);
}
impl Default for CppObj3 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl CppObj3 {
    #[inline]
    pub unsafe fn new(x: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        CppObj3_CppObj3(__bindgen_tmp.as_mut_ptr(), x);
        __bindgen_tmp.assume_init()
    }
}
unsafe extern "C" {
    #[link_name = "\u{1}??1CppObj3@@UEAA@XZ"]
    pub fn CppObj3_CppObj3_destructor(this: *mut CppObj3);
}
#[repr(C)]
#[derive(Debug)]
pub struct CppObj4 {
    pub _base: CppObj2,
    pub x: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of CppObj4"][::std::mem::size_of::<CppObj4>() - 24usize];
    ["Alignment of CppObj4"][::std::mem::align_of::<CppObj4>() - 8usize];
    ["Offset of field: CppObj4::x"][::std::mem::offset_of!(CppObj4, x) - 16usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}??0CppObj4@@QEAA@H@Z"]
    pub fn CppObj4_CppObj4(this: *mut CppObj4, x: ::std::os::raw::c_int);
}
impl Default for CppObj4 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl CppObj4 {
    #[inline]
    pub unsafe fn new(x: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        CppObj4_CppObj4(__bindgen_tmp.as_mut_ptr(), x);
        __bindgen_tmp.assume_init()
    }
}
unsafe extern "C" {
    #[link_name = "\u{1}??1CppObj4@@UEAA@XZ"]
    pub fn CppObj4_CppObj4_destructor(this: *mut CppObj4);
}
