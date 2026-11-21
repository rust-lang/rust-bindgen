#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug)]
pub struct baz_Foo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
pub struct baz_inner_Bar {
    _unused: [u8; 0],
}
pub type Quux<T> = *mut T;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Pair<T, U> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<U>>,
    pub first: *mut T,
    pub second: *mut U,
}
impl<T, U> Default for Pair<T, U> {
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
        "Size of template specialization: Quux_open0_baz_Foo_close0",
    ][::std::mem::size_of::<Quux<baz_Foo>>() - 8usize];
    [
        "Align of template specialization: Quux_open0_baz_Foo_close0",
    ][::std::mem::align_of::<Quux<baz_Foo>>() - 8usize];
};
unsafe extern "C" {
    pub static mut global: Quux<baz_Foo>;
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Quux_open0_baz_inner_Bar_close0",
    ][::std::mem::size_of::<Quux<baz_inner_Bar>>() - 8usize];
    [
        "Align of template specialization: Quux_open0_baz_inner_Bar_close0",
    ][::std::mem::align_of::<Quux<baz_inner_Bar>>() - 8usize];
};
unsafe extern "C" {
    pub static mut nested_global: Quux<baz_inner_Bar>;
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Quux_open0_baz_Foo_close0",
    ][::std::mem::size_of::<Quux<baz_Foo>>() - 8usize];
    [
        "Align of template specialization: Quux_open0_baz_Foo_close0",
    ][::std::mem::align_of::<Quux<baz_Foo>>() - 8usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_Z13returns_aliasi"]
    pub fn returns_alias(x: ::std::os::raw::c_int) -> Quux<baz_Foo>;
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Quux_open0_baz_Foo_close0",
    ][::std::mem::size_of::<Quux<baz_Foo>>() - 8usize];
    [
        "Align of template specialization: Quux_open0_baz_Foo_close0",
    ][::std::mem::align_of::<Quux<baz_Foo>>() - 8usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_Z11takes_aliasPN3baz3FooEi"]
    pub fn takes_alias(a: Quux<baz_Foo>, x: ::std::os::raw::c_int);
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Pair_open0_baz_Foo_baz_inner_Bar_close0",
    ][::std::mem::size_of::<Pair<baz_Foo, baz_inner_Bar>>() - 16usize];
    [
        "Align of template specialization: Pair_open0_baz_Foo_baz_inner_Bar_close0",
    ][::std::mem::align_of::<Pair<baz_Foo, baz_inner_Bar>>() - 8usize];
};
unsafe extern "C" {
    pub static mut pair_global: Pair<baz_Foo, baz_inner_Bar>;
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z12returns_pairi"]
    pub fn returns_pair(x: ::std::os::raw::c_int) -> Pair<baz_Foo, baz_inner_Bar>;
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Pair_open0_baz_Foo_baz_inner_Bar_close0",
    ][::std::mem::size_of::<Pair<baz_Foo, baz_inner_Bar>>() - 16usize];
    [
        "Align of template specialization: Pair_open0_baz_Foo_baz_inner_Bar_close0",
    ][::std::mem::align_of::<Pair<baz_Foo, baz_inner_Bar>>() - 8usize];
};
