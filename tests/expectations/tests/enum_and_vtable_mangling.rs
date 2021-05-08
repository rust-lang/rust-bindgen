#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub const match_: _bindgen_ty_1 = _bindgen_ty_1::match_;
pub const whatever_else: _bindgen_ty_1 = _bindgen_ty_1::whatever_else;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_1 {
    match_ = 0,
    whatever_else = 1,
}
#[repr(C)]
pub struct C__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct C {
    pub vtable_: *const C__bindgen_vtable,
    pub i: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(
        ::std::mem::size_of::<C>(),
        16usize,
        concat!("Size of: ", stringify!(C))
    );
    assert_eq!(
        ::std::mem::align_of::<C>(),
        8usize,
        concat!("Alignment of ", stringify!(C))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<C>())).i as *const _ as usize },
        8usize,
        concat!("Offset of field: ", stringify!(C), "::", stringify!(i))
    );
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
extern "C" {
    #[link_name = "\u{1}_ZN1C5matchEv"]
    pub fn C_match(this: *mut ::std::os::raw::c_void);
}
