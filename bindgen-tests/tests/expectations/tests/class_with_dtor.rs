#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct HandleWithDtor<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub ptr: *mut T,
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
const _: () = {
    ["Size of WithoutDtor"][::std::mem::size_of::<WithoutDtor>() - 8usize];
    ["Alignment of WithoutDtor"][::std::mem::align_of::<WithoutDtor>() - 8usize];
    [
        "Offset of field: WithoutDtor::shouldBeWithDtor",
    ][::std::mem::offset_of!(WithoutDtor, shouldBeWithDtor) - 0usize];
};
impl Default for WithoutDtor {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
const _: () = {
    [
        "Size of template specialization: HandleWithDtor_open0_int_close0",
    ][::std::mem::size_of::<HandleWithDtor<::std::os::raw::c_int>>() - 8usize];
    [
        "Align of template specialization: HandleWithDtor_open0_int_close0",
    ][::std::mem::align_of::<HandleWithDtor<::std::os::raw::c_int>>() - 8usize];
};
