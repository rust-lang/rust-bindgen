#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct typedef_named_struct {
    pub has_name: bool,
}
const _: () = {
    assert!(
        ::std::mem::size_of::<typedef_named_struct>() == 1usize,
        "Size of typedef_named_struct",
    );
    assert!(
        ::std::mem::align_of::<typedef_named_struct>() == 1usize,
        "Alignment of typedef_named_struct",
    );
    assert!(
        ::std::mem::offset_of!(typedef_named_struct, has_name) == 0usize,
        "Offset of field: typedef_named_struct::has_name",
    );
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct _bindgen_ty_1 {
    pub no_name: *mut ::std::os::raw::c_void,
}
const _: () = {
    assert!(::std::mem::size_of::<_bindgen_ty_1>() == 8usize, "Size of _bindgen_ty_1");
    assert!(
        ::std::mem::align_of::<_bindgen_ty_1>() == 8usize,
        "Alignment of _bindgen_ty_1",
    );
    assert!(
        ::std::mem::offset_of!(_bindgen_ty_1, no_name) == 0usize,
        "Offset of field: _bindgen_ty_1::no_name",
    );
};
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
