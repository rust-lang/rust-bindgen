#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub const NSID_LENGTH: u32 = 10;
#[repr(C)]
pub struct nsID__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsID {
    pub vtable_: *const nsID__bindgen_vtable,
}
#[test]
fn bindgen_test_layout_nsID() {
    assert_eq!(
        ::std::mem::size_of::<nsID>(),
        8usize,
        concat!("Size of: ", stringify!(nsID))
    );
    assert_eq!(
        ::std::mem::align_of::<nsID>(),
        8usize,
        concat!("Alignment of ", stringify!(nsID))
    );
}
impl Default for nsID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_nsID {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_nsID {}
impl Drop for Box_nsID {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(8usize, 8usize).unwrap(),
            );
        }
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN4nsID16ToProvidedStringERA10_c"]
    pub fn nsID_ToProvidedString(
        this: *mut ::std::os::raw::c_void,
        aDest: *mut [::std::os::raw::c_char; 10usize],
    );
}
