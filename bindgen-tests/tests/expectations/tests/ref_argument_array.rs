#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const NSID_LENGTH: u32 = 10;
#[repr(C)]
pub struct nsID__bindgen_vtable {
    pub nsID_ToProvidedString: unsafe extern "C" fn(
        this: *mut nsID,
        aDest: *mut [::std::os::raw::c_char; 10usize],
    ),
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsID {
    pub vtable_: *const nsID__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of nsID"][::std::mem::size_of::<nsID>() - 8usize];
    ["Alignment of nsID"][::std::mem::align_of::<nsID>() - 8usize];
};
impl Default for nsID {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
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
