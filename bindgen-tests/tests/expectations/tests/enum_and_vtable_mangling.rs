#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const match_: _bindgen_ty_1 = _bindgen_ty_1::match_;
pub const whatever_else: _bindgen_ty_1 = _bindgen_ty_1::whatever_else;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_1 {
    match_ = 0,
    whatever_else = 1,
}
#[repr(C)]
pub struct C__bindgen_vtable {
    pub C_match: unsafe extern "C" fn(this: *mut C),
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct C {
    pub vtable_: *const C__bindgen_vtable,
    pub i: ::std::os::raw::c_int,
}
const _: () = {
    ["Size of C"][::std::mem::size_of::<C>() - 16usize];
    ["Alignment of C"][::std::mem::align_of::<C>() - 8usize];
    ["Offset of field: C::i"][::std::mem::offset_of!(C, i) - 8usize];
};
impl Default for C {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN1C5matchEv"]
    pub fn C_match(this: *mut ::std::os::raw::c_void);
}
