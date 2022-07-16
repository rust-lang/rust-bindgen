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
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
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
    const UNINIT: ::std::mem::MaybeUninit<WithoutDtor> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
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
            ::std::ptr::addr_of!((*ptr).shouldBeWithDtor) as usize -
                ptr as usize
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
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
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
