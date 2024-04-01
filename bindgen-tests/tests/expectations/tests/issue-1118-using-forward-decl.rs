#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type c = nsTArray;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsTArray_base {
    pub d: *mut ::std::os::raw::c_int,
}
const _: () = {
    ["Size of nsTArray_base"][::std::mem::size_of::<nsTArray_base>() - 8usize];
    ["Alignment of nsTArray_base"][::std::mem::align_of::<nsTArray_base>() - 8usize];
    [
        "Offset of field: nsTArray_base::d",
    ][::std::mem::offset_of!(nsTArray_base, d) - 0usize];
};
impl Default for nsTArray_base {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsTArray {
    pub _base: nsTArray_base,
}
impl Default for nsTArray {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsIContent {
    pub foo: nsTArray,
}
const _: () = {
    ["Size of nsIContent"][::std::mem::size_of::<nsIContent>() - 8usize];
    ["Alignment of nsIContent"][::std::mem::align_of::<nsIContent>() - 8usize];
    [
        "Offset of field: nsIContent::foo",
    ][::std::mem::offset_of!(nsIContent, foo) - 0usize];
};
impl Default for nsIContent {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    #[link_name = "\u{1}_Z35Gecko_GetAnonymousContentForElementv"]
    pub fn Gecko_GetAnonymousContentForElement() -> *mut nsTArray;
}
const _: () = {
    [
        "Size of template specialization: nsTArray_open0_ptr_nsIContent_close0",
    ][::std::mem::size_of::<nsTArray>() - 8usize];
    [
        "Align of template specialization: nsTArray_open0_ptr_nsIContent_close0",
    ][::std::mem::align_of::<nsTArray>() - 8usize];
};
const _: () = {
    [
        "Size of template specialization: nsTArray_open0_ptr_nsIContent_close0",
    ][::std::mem::size_of::<nsTArray>() - 8usize];
    [
        "Align of template specialization: nsTArray_open0_ptr_nsIContent_close0",
    ][::std::mem::align_of::<nsTArray>() - 8usize];
};
