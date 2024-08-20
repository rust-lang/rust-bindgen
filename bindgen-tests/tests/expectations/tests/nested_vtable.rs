#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
pub struct nsISupports__bindgen_vtable {
    pub nsISupports_QueryInterface: unsafe extern "C" fn(
        this: *mut nsISupports,
    ) -> *mut nsISupports,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsISupports {
    pub vtable_: *const nsISupports__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of nsISupports"][::std::mem::size_of::<nsISupports>() - 8usize];
    ["Alignment of nsISupports"][::std::mem::align_of::<nsISupports>() - 8usize];
};
impl Default for nsISupports {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN11nsISupports14QueryInterfaceEv"]
    pub fn nsISupports_QueryInterface(
        this: *mut ::std::os::raw::c_void,
    ) -> *mut nsISupports;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsIRunnable {
    pub _base: nsISupports,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of nsIRunnable"][::std::mem::size_of::<nsIRunnable>() - 8usize];
    ["Alignment of nsIRunnable"][::std::mem::align_of::<nsIRunnable>() - 8usize];
};
impl Default for nsIRunnable {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Runnable {
    pub _base: nsIRunnable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Runnable"][::std::mem::size_of::<Runnable>() - 8usize];
    ["Alignment of Runnable"][::std::mem::align_of::<Runnable>() - 8usize];
};
impl Default for Runnable {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
