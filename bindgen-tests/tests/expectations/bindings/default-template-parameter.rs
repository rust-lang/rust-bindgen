#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo<T, U> {
    pub t: T,
    pub u: U,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<U>>,
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
#[test]
fn __bindgen_test_layout_Foo_open0_bool__int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<Foo<bool, ::std::os::raw::c_int>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify ! (Foo < bool , :: std :: os :: raw :: c_int >)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<Foo<bool, ::std::os::raw::c_int>>(),
        4usize,
        concat!(
            "Alignment of template specialization: ",
            stringify ! (Foo < bool , :: std :: os :: raw :: c_int >)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}_ZL3bar"]
    pub static mut bar: Foo<bool, ::std::os::raw::c_int>;
}
