#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
pub struct Foo__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug)]
pub struct Foo {
    pub vtable_: *const Foo__bindgen_vtable,
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(
        ::std::mem::size_of::<Foo>(),
        8usize,
        concat!("Size of: ", stringify!(Foo))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo>(),
        8usize,
        concat!("Alignment of ", stringify!(Foo))
    );
}
impl Default for Foo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_Foo {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Foo {}
impl Drop for Box_Foo {
    fn drop(&mut self) {
        unsafe {
            Foo_Foo_destructor(self.ptr as *mut Foo);
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(8usize, 8usize).unwrap(),
            );
        }
    }
}
