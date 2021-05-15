#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub struct BlocklistMe(u8);

/// Because this type contains a blocklisted type, it should not derive Debug.
#[repr(C)]
pub struct ShouldManuallyImplDebug {
    pub a: BlocklistMe,
}
#[test]
fn bindgen_test_layout_ShouldManuallyImplDebug() {
    assert_eq!(
        ::std::mem::size_of::<ShouldManuallyImplDebug>(),
        1usize,
        concat!("Size of: ", stringify!(ShouldManuallyImplDebug))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldManuallyImplDebug>(),
        1usize,
        concat!("Alignment of ", stringify!(ShouldManuallyImplDebug))
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize =
                std::mem::size_of::<ShouldManuallyImplDebug>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], ShouldManuallyImplDebug>(
                    buffer,
                )
            };
            let struct_ptr = &struct_instance as *const ShouldManuallyImplDebug;
            let field_ptr = std::ptr::addr_of!(struct_instance.a);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldManuallyImplDebug),
            "::",
            stringify!(a)
        )
    );
}
impl Default for ShouldManuallyImplDebug {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for ShouldManuallyImplDebug {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "ShouldManuallyImplDebug {{  }}")
    }
}
