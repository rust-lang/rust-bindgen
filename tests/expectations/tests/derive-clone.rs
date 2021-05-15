#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// This struct should derive `Clone`.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShouldDeriveClone {
    pub large: [::std::os::raw::c_int; 33usize],
}
#[test]
fn bindgen_test_layout_ShouldDeriveClone() {
    assert_eq!(
        ::std::mem::size_of::<ShouldDeriveClone>(),
        132usize,
        concat!("Size of: ", stringify!(ShouldDeriveClone))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldDeriveClone>(),
        4usize,
        concat!("Alignment of ", stringify!(ShouldDeriveClone))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<ShouldDeriveClone>() };
            let struct_ptr = &struct_instance as *const ShouldDeriveClone;
            let field_ptr = std::ptr::addr_of!(struct_instance.large);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldDeriveClone),
            "::",
            stringify!(large)
        )
    );
}
impl Default for ShouldDeriveClone {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
