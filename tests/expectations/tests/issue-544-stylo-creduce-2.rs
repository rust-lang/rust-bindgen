#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo<T>
where
    T: __bindgen_has_inner_type_Associated,
{
    pub member: Foo_SecondAlias<i32>,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
pub type Foo_FirstAlias<T> =
    <T as __bindgen_has_inner_type_Associated>::Associated;
pub type Foo_SecondAlias<T> = Foo<Foo_FirstAlias<T>>;
impl<T> Default for Foo<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub trait __bindgen_has_inner_type_Associated {
    type Associated: std::fmt::Debug + Default + Copy + Clone;
}
