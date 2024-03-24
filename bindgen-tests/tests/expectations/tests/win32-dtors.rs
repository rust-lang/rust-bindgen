#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default)]
pub struct CppObj {
    pub x: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_CppObj() {
    const UNINIT: ::std::mem::MaybeUninit<CppObj> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<CppObj>(), 4usize, "Size of CppObj");
    assert_eq!(::std::mem::align_of::<CppObj>(), 4usize, "Alignment of CppObj");
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        "Offset of field: CppObj::x",
    );
}
extern "C" {
    #[link_name = "\u{1}??0CppObj@@QEAA@H@Z"]
    pub fn CppObj_CppObj(this: *mut CppObj, x: ::std::os::raw::c_int);
}
extern "C" {
    #[link_name = "\u{1}??1CppObj@@QEAA@XZ"]
    pub fn CppObj_CppObj_destructor(this: *mut CppObj);
}
impl CppObj {
    #[inline]
    pub unsafe fn new(x: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        CppObj_CppObj(&mut __bindgen_tmp, x);
        __bindgen_tmp
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
#[test]
fn bindgen_test_layout_CppObj2() {
    const UNINIT: ::std::mem::MaybeUninit<CppObj2> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<CppObj2>(), 16usize, "Size of CppObj2");
    assert_eq!(::std::mem::align_of::<CppObj2>(), 8usize, "Alignment of CppObj2");
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        8usize,
        "Offset of field: CppObj2::x",
    );
}
extern "C" {
    #[link_name = "\u{1}??0CppObj2@@QEAA@H@Z"]
    pub fn CppObj2_CppObj2(this: *mut CppObj2, x: ::std::os::raw::c_int);
}
impl Default for CppObj2 {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
impl CppObj2 {
    #[inline]
    pub unsafe fn new(x: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        CppObj2_CppObj2(&mut __bindgen_tmp, x);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}??1CppObj2@@UEAA@XZ"]
    pub fn CppObj2_CppObj2_destructor(this: *mut CppObj2);
}
#[repr(C)]
#[derive(Debug)]
pub struct CppObj3 {
    pub _base: CppObj2,
    pub x: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_CppObj3() {
    const UNINIT: ::std::mem::MaybeUninit<CppObj3> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<CppObj3>(), 24usize, "Size of CppObj3");
    assert_eq!(::std::mem::align_of::<CppObj3>(), 8usize, "Alignment of CppObj3");
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        16usize,
        "Offset of field: CppObj3::x",
    );
}
extern "C" {
    #[link_name = "\u{1}??0CppObj3@@QEAA@H@Z"]
    pub fn CppObj3_CppObj3(this: *mut CppObj3, x: ::std::os::raw::c_int);
}
impl Default for CppObj3 {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
impl CppObj3 {
    #[inline]
    pub unsafe fn new(x: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        CppObj3_CppObj3(&mut __bindgen_tmp, x);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}??1CppObj3@@UEAA@XZ"]
    pub fn CppObj3_CppObj3_destructor(this: *mut CppObj3);
}
#[repr(C)]
#[derive(Debug)]
pub struct CppObj4 {
    pub _base: CppObj2,
    pub x: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_CppObj4() {
    const UNINIT: ::std::mem::MaybeUninit<CppObj4> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<CppObj4>(), 24usize, "Size of CppObj4");
    assert_eq!(::std::mem::align_of::<CppObj4>(), 8usize, "Alignment of CppObj4");
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        16usize,
        "Offset of field: CppObj4::x",
    );
}
extern "C" {
    #[link_name = "\u{1}??0CppObj4@@QEAA@H@Z"]
    pub fn CppObj4_CppObj4(this: *mut CppObj4, x: ::std::os::raw::c_int);
}
impl Default for CppObj4 {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
impl CppObj4 {
    #[inline]
    pub unsafe fn new(x: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        CppObj4_CppObj4(&mut __bindgen_tmp, x);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}??1CppObj4@@UEAA@XZ"]
    pub fn CppObj4_CppObj4_destructor(this: *mut CppObj4);
}
