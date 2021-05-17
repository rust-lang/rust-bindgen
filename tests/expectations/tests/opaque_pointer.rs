#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// <div rustbindgen opaque></div>
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OtherOpaque {
    pub _bindgen_opaque_blob: u32,
}
#[test]
fn bindgen_test_layout_OtherOpaque() {
    assert_eq!(
        ::std::mem::size_of::<OtherOpaque>(),
        4usize,
        concat!("Size of: ", stringify!(OtherOpaque))
    );
    assert_eq!(
        ::std::mem::align_of::<OtherOpaque>(),
        4usize,
        concat!("Alignment of ", stringify!(OtherOpaque))
    );
}
/// <div rustbindgen opaque></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Opaque {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq)]
pub struct WithOpaquePtr {
    pub whatever: *mut u8,
    pub other: u32,
    pub t: OtherOpaque,
}
#[test]
fn bindgen_test_layout_WithOpaquePtr() {
    assert_eq!(
        ::std::mem::size_of::<WithOpaquePtr>(),
        16usize,
        concat!("Size of: ", stringify!(WithOpaquePtr))
    );
    assert_eq!(
        ::std::mem::align_of::<WithOpaquePtr>(),
        8usize,
        concat!("Alignment of ", stringify!(WithOpaquePtr))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<WithOpaquePtr>() };
            let struct_ptr = &struct_instance as *const WithOpaquePtr;
            let field_ptr = std::ptr::addr_of!(struct_instance.whatever);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithOpaquePtr),
            "::",
            stringify!(whatever)
        )
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<WithOpaquePtr>() };
            let struct_ptr = &struct_instance as *const WithOpaquePtr;
            let field_ptr = std::ptr::addr_of!(struct_instance.other);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(WithOpaquePtr),
            "::",
            stringify!(other)
        )
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<WithOpaquePtr>() };
            let struct_ptr = &struct_instance as *const WithOpaquePtr;
            let field_ptr = std::ptr::addr_of!(struct_instance.t);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(WithOpaquePtr),
            "::",
            stringify!(t)
        )
    );
}
impl Default for WithOpaquePtr {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
