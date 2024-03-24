#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
const _: () = {
    ["Size of C"][::std::mem::size_of::<C>() - 72usize];
    ["Alignment of C"][::std::mem::align_of::<C>() - 8usize];
    ["Offset of field: C::c"][::std::mem::offset_of!(C, c) - 0usize];
    ["Offset of field: C::ptr"][::std::mem::offset_of!(C, ptr) - 8usize];
    ["Offset of field: C::arr"][::std::mem::offset_of!(C, arr) - 16usize];
    ["Offset of field: C::d"][::std::mem::offset_of!(C, d) - 56usize];
    ["Offset of field: C::other_ptr"][::std::mem::offset_of!(C, other_ptr) - 64usize];
};
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
const _: () = {
    ["Size of D"][::std::mem::size_of::<D>() - 80usize];
    ["Alignment of D"][::std::mem::align_of::<D>() - 8usize];
    ["Offset of field: D::ptr"][::std::mem::offset_of!(D, ptr) - 72usize];
};
impl Default for D {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
