#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![cfg(feature = "nightly")]
#[repr(C)]
pub struct PackedVtable__bindgen_vtable(::std::os::raw::c_void);
#[repr(C, packed)]
pub struct PackedVtable {
    pub vtable_: *const PackedVtable__bindgen_vtable,
}
#[test]
fn bindgen_test_layout_PackedVtable() {
    assert_eq!(::std::mem::size_of::<PackedVtable>(), 8usize, "Size of PackedVtable");
    assert_eq!(
        ::std::mem::align_of::<PackedVtable>(),
        1usize,
        "Alignment of PackedVtable",
    );
}
impl Default for PackedVtable {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN12PackedVtableD1Ev"]
    pub fn PackedVtable_PackedVtable_destructor(this: *mut PackedVtable);
}
