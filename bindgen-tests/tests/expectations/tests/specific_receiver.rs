#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
pub struct Fish__bindgen_vtable {
    pub Fish_swim: unsafe extern "C" fn(this: *mut Fish),
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fish {
    pub vtable_: *const Fish__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Fish"][::std::mem::size_of::<Fish>() - 8usize];
    ["Alignment of Fish"][::std::mem::align_of::<Fish>() - 8usize];
};
impl Default for Fish {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    #[link_name = "_ZN4Fish4swimEv"]
    pub fn Fish_swim(this: *mut Fish);
}
