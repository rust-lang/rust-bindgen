#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo<T, U> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<U>>,
    pub t: T,
    pub u: U,
}
impl<T, U> Default for Foo<T, U> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Foo_open0_bool__int_close0",
    ][::std::mem::size_of::<Foo<bool, ::std::os::raw::c_int>>() - 8usize];
    [
        "Align of template specialization: Foo_open0_bool__int_close0",
    ][::std::mem::align_of::<Foo<bool, ::std::os::raw::c_int>>() - 4usize];
};
extern "C" {
    #[link_name = "\u{1}_ZL3bar"]
    pub static mut bar: Foo<bool, ::std::os::raw::c_int>;
}
