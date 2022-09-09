#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct typedef_named_struct {
    pub has_name: bool,
}
#[test]
fn bindgen_test_layout_typedef_named_struct() {
    const UNINIT: ::std::mem::MaybeUninit<typedef_named_struct> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<typedef_named_struct>(),
        1usize,
        concat!("Size of: ", stringify!(typedef_named_struct))
    );
    assert_eq!(
        ::std::mem::align_of::<typedef_named_struct>(),
        1usize,
        concat!("Alignment of ", stringify!(typedef_named_struct))
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).has_name) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(typedef_named_struct),
            "::",
            stringify!(has_name)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct _bindgen_ty_1 {
    pub no_name: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<_bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(_bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<_bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(_bindgen_ty_1))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).no_name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bindgen_ty_1),
            "::",
            stringify!(no_name)
        )
    );
}
impl Default for _bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type struct_ptr_t = *mut _bindgen_ty_1;
pub type struct_ptr_ptr_t = *mut *mut _bindgen_ty_1;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum typedef_named_enum {
    ENUM_HAS_NAME = 1,
}
pub const ENUM_IS_ANON: _bindgen_ty_2 = _bindgen_ty_2::ENUM_IS_ANON;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_2 {
    ENUM_IS_ANON = 0,
}
pub type enum_ptr_t = *mut _bindgen_ty_2;
pub type enum_ptr_ptr_t = *mut *mut _bindgen_ty_2;
