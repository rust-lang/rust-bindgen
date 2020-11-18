#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
pub struct C__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct C {
    pub vtable_: *const C__bindgen_vtable,
}
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(
        ::std::mem::size_of::<C>(),
        8usize,
        concat!("Size of: ", stringify!(C))
    );
    assert_eq!(
        ::std::mem::align_of::<C>(),
        8usize,
        concat!("Alignment of ", stringify!(C))
    );
}
impl Default for C {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_C {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_C {}
impl Drop for Box_C {
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
    #[link_name = "\u{1}_ZN1C8do_thingEc"]
    pub fn C_do_thing(
        this: *mut ::std::os::raw::c_void,
        arg1: ::std::os::raw::c_char,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN1C8do_thingEi"]
    pub fn C_do_thing1(
        this: *mut ::std::os::raw::c_void,
        arg1: ::std::os::raw::c_int,
    );
}
