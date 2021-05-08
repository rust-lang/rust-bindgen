#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
pub struct nsISupports__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsISupports {
    pub vtable_: *const nsISupports__bindgen_vtable,
}
#[test]
fn bindgen_test_layout_nsISupports() {
    assert_eq!(
        ::std::mem::size_of::<nsISupports>(),
        8usize,
        concat!("Size of: ", stringify!(nsISupports))
    );
    assert_eq!(
        ::std::mem::align_of::<nsISupports>(),
        8usize,
        concat!("Alignment of ", stringify!(nsISupports))
    );
}
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
#[test]
fn bindgen_test_layout_nsIRunnable() {
    assert_eq!(
        ::std::mem::size_of::<nsIRunnable>(),
        8usize,
        concat!("Size of: ", stringify!(nsIRunnable))
    );
    assert_eq!(
        ::std::mem::align_of::<nsIRunnable>(),
        8usize,
        concat!("Alignment of ", stringify!(nsIRunnable))
    );
}
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
#[test]
fn bindgen_test_layout_Runnable() {
    assert_eq!(
        ::std::mem::size_of::<Runnable>(),
        8usize,
        concat!("Size of: ", stringify!(Runnable))
    );
    assert_eq!(
        ::std::mem::align_of::<Runnable>(),
        8usize,
        concat!("Alignment of ", stringify!(Runnable))
    );
}
impl Default for Runnable {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
