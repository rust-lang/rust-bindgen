#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type AnotherInt = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct C {
    pub c: C_MyInt,
    pub ptr: *mut C_MyInt,
    pub arr: [C_MyInt; 10usize],
    pub d: AnotherInt,
    pub other_ptr: *mut AnotherInt,
}
pub type C_MyInt = ::std::os::raw::c_int;
pub type C_Lookup = *const ::std::os::raw::c_char;
#[test]
fn bindgen_test_layout_C() {
    const UNINIT: ::std::mem::MaybeUninit<C> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<C>(),
        72usize,
        concat!("Size of: ", stringify!(C))
    );
    assert_eq!(
        ::std::mem::align_of::<C>(),
        8usize,
        concat!("Alignment of ", stringify!(C))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).c) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(C), "::", stringify!(c))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ptr) as usize - ptr as usize },
        8usize,
        concat!("Offset of field: ", stringify!(C), "::", stringify!(ptr))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).arr) as usize - ptr as usize },
        16usize,
        concat!("Offset of field: ", stringify!(C), "::", stringify!(arr))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).d) as usize - ptr as usize },
        56usize,
        concat!("Offset of field: ", stringify!(C), "::", stringify!(d))
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).other_ptr) as usize - ptr as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(C),
            "::",
            stringify!(other_ptr)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN1C6methodEi"]
    pub fn C_method(this: *mut C, c: C_MyInt);
}
extern "C" {
    #[link_name = "\u{1}_ZN1C9methodRefERi"]
    pub fn C_methodRef(this: *mut C, c: *mut C_MyInt);
}
extern "C" {
    #[link_name = "\u{1}_ZN1C16complexMethodRefERPKc"]
    pub fn C_complexMethodRef(this: *mut C, c: *mut C_Lookup);
}
extern "C" {
    #[link_name = "\u{1}_ZN1C13anotherMethodEi"]
    pub fn C_anotherMethod(this: *mut C, c: AnotherInt);
}
impl Default for C {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl C {
    #[inline]
    pub unsafe fn method(&mut self, c: C_MyInt) {
        C_method(self, c)
    }
    #[inline]
    pub unsafe fn methodRef(&mut self, c: *mut C_MyInt) {
        C_methodRef(self, c)
    }
    #[inline]
    pub unsafe fn complexMethodRef(&mut self, c: *mut C_Lookup) {
        C_complexMethodRef(self, c)
    }
    #[inline]
    pub unsafe fn anotherMethod(&mut self, c: AnotherInt) {
        C_anotherMethod(self, c)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct D {
    pub _base: C,
    pub ptr: *mut C_MyInt,
}
#[test]
fn bindgen_test_layout_D() {
    const UNINIT: ::std::mem::MaybeUninit<D> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<D>(),
        80usize,
        concat!("Size of: ", stringify!(D))
    );
    assert_eq!(
        ::std::mem::align_of::<D>(),
        8usize,
        concat!("Alignment of ", stringify!(D))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ptr) as usize - ptr as usize },
        72usize,
        concat!("Offset of field: ", stringify!(D), "::", stringify!(ptr))
    );
}
impl Default for D {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
