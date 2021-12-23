#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type c = nsTArray;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsTArray_base {
    pub d: *mut ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_nsTArray_base() {
    assert_eq!(
        ::std::mem::size_of::<nsTArray_base>(),
        8usize,
        concat!("Size of: ", stringify!(nsTArray_base))
    );
    assert_eq!(
        ::std::mem::align_of::<nsTArray_base>(),
        8usize,
        concat!("Alignment of ", stringify!(nsTArray_base))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nsTArray_base>())).d as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nsTArray_base),
            "::",
            stringify!(d)
        )
    );
}
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
#[test]
fn bindgen_test_layout_nsIContent() {
    assert_eq!(
        ::std::mem::size_of::<nsIContent>(),
        8usize,
        concat!("Size of: ", stringify!(nsIContent))
    );
    assert_eq!(
        ::std::mem::align_of::<nsIContent>(),
        8usize,
        concat!("Alignment of ", stringify!(nsIContent))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nsIContent>())).foo as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nsIContent),
            "::",
            stringify!(foo)
        )
    );
}
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
#[test]
fn __bindgen_test_layout_nsTArray_open0_ptr_nsIContent_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<nsTArray>(),
        8usize,
        concat!("Size of template specialization: ", stringify!(nsTArray))
    );
    assert_eq!(
        ::std::mem::align_of::<nsTArray>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(nsTArray)
        )
    );
}
#[test]
fn __bindgen_test_layout_nsTArray_open0_ptr_nsIContent_close0_instantiation_1()
{
    assert_eq!(
        ::std::mem::size_of::<nsTArray>(),
        8usize,
        concat!("Size of template specialization: ", stringify!(nsTArray))
    );
    assert_eq!(
        ::std::mem::align_of::<nsTArray>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(nsTArray)
        )
    );
}
