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
    fn test_field_whatever() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WithOpaquePtr>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).whatever) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(WithOpaquePtr),
                "::",
                stringify!(whatever)
            )
        );
    }
    test_field_whatever();
    fn test_field_other() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WithOpaquePtr>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).other) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(WithOpaquePtr),
                "::",
                stringify!(other)
            )
        );
    }
    test_field_other();
    fn test_field_t() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WithOpaquePtr>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).t) as usize - ptr as usize
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
    test_field_t();
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
