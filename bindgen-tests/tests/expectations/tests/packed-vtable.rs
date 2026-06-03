#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![cfg(feature = "nightly")]
#[repr(C)]
pub struct PackedVtable__bindgen_vtable(::std::os::raw::c_void);
#[repr(C, packed)]
pub struct PackedVtable {
    pub vtable_: *const PackedVtable__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of PackedVtable"][::std::mem::size_of::<PackedVtable>() - 8usize];
    ["Alignment of PackedVtable"][::std::mem::align_of::<PackedVtable>() - 1usize];
};
impl Default for PackedVtable {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    #[link_name = "_ZN12PackedVtableD1Ev"]
    pub fn PackedVtable_PackedVtable_destructor(this: *mut PackedVtable);
}
