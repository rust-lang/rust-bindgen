#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![cfg(target_os = "macos")]

extern crate block;
extern "C" {
    #[link_name = "\u{1}_Z8atexit_bU13block_pointerFvvE"]
    pub fn atexit_b(arg1: _bindgen_ty_id_33);
}
pub type dispatch_data_t = *mut ::std::os::raw::c_void;
pub type dispatch_data_applier_t = _bindgen_ty_id_40;
extern "C" {
    #[link_name = "\u{1}_Z19dispatch_data_applyPvU13block_pointerFbS_yPKvyE"]
    pub fn dispatch_data_apply(
        data: dispatch_data_t,
        applier: dispatch_data_applier_t,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_Z3fooU13block_pointerFvyE"]
    pub fn foo(arg1: _bindgen_ty_id_50) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_Z7foo_ptrPU13block_pointerFvyE"]
    pub fn foo_ptr(arg1: *mut _bindgen_ty_id_56) -> bool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct contains_block_pointers {
    pub val: contains_block_pointers__bindgen_ty_id_61,
    pub ptr_val: *mut _bindgen_ty_id_68,
}
#[test]
fn bindgen_test_layout_contains_block_pointers() {
    const UNINIT: ::std::mem::MaybeUninit<contains_block_pointers> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<contains_block_pointers>(),
        16usize,
        concat!("Size of: ", stringify!(contains_block_pointers))
    );
    assert_eq!(
        ::std::mem::align_of::<contains_block_pointers>(),
        8usize,
        concat!("Alignment of ", stringify!(contains_block_pointers))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).val) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(contains_block_pointers),
            "::",
            stringify!(val)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ptr_val) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(contains_block_pointers),
            "::",
            stringify!(ptr_val)
        )
    );
}
impl Default for contains_block_pointers {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type _bindgen_ty_id_33 = *const ::block::Block<(), ()>;
pub type _bindgen_ty_id_40 = *const ::block::Block<
    (dispatch_data_t, usize, *const ::std::os::raw::c_void, usize),
    bool,
>;
pub type _bindgen_ty_id_50 = *const ::block::Block<(usize,), ()>;
pub type _bindgen_ty_id_56 = *const ::block::Block<(usize,), ()>;
pub type contains_block_pointers__bindgen_ty_id_61 =
    *const ::block::Block<(::std::os::raw::c_int,), ()>;
pub type _bindgen_ty_id_68 =
    *const ::block::Block<(::std::os::raw::c_int,), ()>;
