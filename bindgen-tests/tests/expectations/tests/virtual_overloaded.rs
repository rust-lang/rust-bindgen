#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
pub struct C__bindgen_vtable {
    pub C_do_thing: unsafe extern "C" fn(this: *mut C, arg1: ::std::os::raw::c_char),
    pub C_do_thing1: unsafe extern "C" fn(this: *mut C, arg1: ::std::os::raw::c_int),
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct C {
    pub vtable_: *const C__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of C"][::std::mem::size_of::<C>() - 8usize];
    ["Alignment of C"][::std::mem::align_of::<C>() - 8usize];
};
impl Default for C {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN1C8do_thingEc"]
    pub fn C_do_thing(this: *mut ::std::os::raw::c_void, arg1: ::std::os::raw::c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZN1C8do_thingEi"]
    pub fn C_do_thing1(this: *mut ::std::os::raw::c_void, arg1: ::std::os::raw::c_int);
}
