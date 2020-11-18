#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct HandleWithDtor<T> {
    pub ptr: *mut T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for HandleWithDtor<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type HandleValue = HandleWithDtor<::std::os::raw::c_int>;
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct WithoutDtor {
    pub shouldBeWithDtor: HandleValue,
}
#[test]
fn bindgen_test_layout_WithoutDtor() {
    assert_eq!(
        ::std::mem::size_of::<WithoutDtor>(),
        8usize,
        concat!("Size of: ", stringify!(WithoutDtor))
    );
    assert_eq!(
        ::std::mem::align_of::<WithoutDtor>(),
        8usize,
        concat!("Alignment of ", stringify!(WithoutDtor))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WithoutDtor>())).shouldBeWithDtor as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithoutDtor),
            "::",
            stringify!(shouldBeWithDtor)
        )
    );
}
impl Default for WithoutDtor {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_WithoutDtor {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_WithoutDtor {}
impl Drop for Box_WithoutDtor {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(8usize, 8usize).unwrap(),
            );
        }
    }
}
#[test]
fn __bindgen_test_layout_HandleWithDtor_open0_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<HandleWithDtor<::std::os::raw::c_int>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(HandleWithDtor<::std::os::raw::c_int>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<HandleWithDtor<::std::os::raw::c_int>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(HandleWithDtor<::std::os::raw::c_int>)
        )
    );
}
