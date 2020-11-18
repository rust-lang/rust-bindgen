#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub struct BlacklistMe(u8);

/// Because this type contains a blacklisted type, it should not derive Hash.
#[repr(C)]
pub struct ShouldNotDeriveHash {
    pub a: BlacklistMe,
}
#[test]
fn bindgen_test_layout_ShouldNotDeriveHash() {
    assert_eq!(
        ::std::mem::size_of::<ShouldNotDeriveHash>(),
        1usize,
        concat!("Size of: ", stringify!(ShouldNotDeriveHash))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldNotDeriveHash>(),
        1usize,
        concat!("Alignment of ", stringify!(ShouldNotDeriveHash))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ShouldNotDeriveHash>())).a as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldNotDeriveHash),
            "::",
            stringify!(a)
        )
    );
}
impl Default for ShouldNotDeriveHash {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_ShouldNotDeriveHash {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_ShouldNotDeriveHash {}
impl Drop for Box_ShouldNotDeriveHash {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
        }
    }
}
