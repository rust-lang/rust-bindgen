#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![cfg(feature = "nightly")]

#[repr(C)]
pub struct PackedVtable__bindgen_vtable(::std::os::raw::c_void);
#[repr(C, packed)]
#[derive(Debug)]
pub struct PackedVtable {
    pub vtable_: *const PackedVtable__bindgen_vtable,
}
#[test]
fn bindgen_test_layout_PackedVtable() {
    assert_eq!(
        ::std::mem::size_of::<PackedVtable>(),
        8usize,
        concat!("Size of: ", stringify!(PackedVtable))
    );
    assert_eq!(
        ::std::mem::align_of::<PackedVtable>(),
        1usize,
        concat!("Alignment of ", stringify!(PackedVtable))
    );
}
impl Default for PackedVtable {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_PackedVtable {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_PackedVtable {}
impl Drop for Box_PackedVtable {
    fn drop(&mut self) {
        unsafe {
            PackedVtable_PackedVtable_destructor(self.ptr as *mut PackedVtable);
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(8usize, 1usize).unwrap(),
            );
        }
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN12PackedVtableD1Ev"]
    pub fn PackedVtable_PackedVtable_destructor(this: *mut PackedVtable);
}
