#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![cfg(target_os = "macos")]

pub type size_t = ::std::os::raw::c_ulonglong;
extern "C" {
    #[link_name = "\u{1}_Z8atexit_bU13block_pointerFvvE"]
    pub fn atexit_b(arg1: *mut ::std::os::raw::c_void);
}
pub type dispatch_data_t = *mut ::std::os::raw::c_void;
pub type dispatch_data_applier_t = *mut ::std::os::raw::c_void;
extern "C" {
    #[link_name = "\u{1}_Z19dispatch_data_applyPvU13block_pointerFbS_yPKvyE"]
    pub fn dispatch_data_apply(
        data: dispatch_data_t,
        applier: dispatch_data_applier_t,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_Z3fooU13block_pointerFvyE"]
    pub fn foo(arg1: *mut ::std::os::raw::c_void) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_Z7foo_ptrPU13block_pointerFvyE"]
    pub fn foo_ptr(arg1: *mut *mut ::std::os::raw::c_void) -> bool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct contains_block_pointers {
    pub val: *mut ::std::os::raw::c_void,
    pub ptr_val: *mut *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_contains_block_pointers() {
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
        unsafe {
            &(*(::std::ptr::null::<contains_block_pointers>())).val as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(contains_block_pointers),
            "::",
            stringify!(val)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<contains_block_pointers>())).ptr_val
                as *const _ as usize
        },
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
