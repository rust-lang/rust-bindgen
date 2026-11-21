#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug)]
pub struct Foo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
pub struct Bar {
    _unused: [u8; 0],
}
pub type Quux<T> = *mut T;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Quux_open0_Foo_close0",
    ][::std::mem::size_of::<Quux<Foo>>() - 8usize];
    [
        "Align of template specialization: Quux_open0_Foo_close0",
    ][::std::mem::align_of::<Quux<Foo>>() - 8usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_Z9no_paramsv"]
    pub fn no_params() -> Quux<Foo>;
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Quux_open0_Foo_close0",
    ][::std::mem::size_of::<Quux<Foo>>() - 8usize];
    [
        "Align of template specialization: Quux_open0_Foo_close0",
    ][::std::mem::align_of::<Quux<Foo>>() - 8usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_Z9one_parami"]
    pub fn one_param(x: ::std::os::raw::c_int) -> Quux<Foo>;
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Quux_open0_Foo_close0",
    ][::std::mem::size_of::<Quux<Foo>>() - 8usize];
    [
        "Align of template specialization: Quux_open0_Foo_close0",
    ][::std::mem::align_of::<Quux<Foo>>() - 8usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_Z10two_paramsii"]
    pub fn two_params(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int) -> Quux<Foo>;
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Quux_open0_Bar_close0",
    ][::std::mem::size_of::<Quux<Bar>>() - 8usize];
    [
        "Align of template specialization: Quux_open0_Bar_close0",
    ][::std::mem::align_of::<Quux<Bar>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Quux_open0_Foo_close0",
    ][::std::mem::size_of::<Quux<Foo>>() - 8usize];
    [
        "Align of template specialization: Quux_open0_Foo_close0",
    ][::std::mem::align_of::<Quux<Foo>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Quux_open0_Foo_close0",
    ][::std::mem::size_of::<Quux<Foo>>() - 8usize];
    [
        "Align of template specialization: Quux_open0_Foo_close0",
    ][::std::mem::align_of::<Quux<Foo>>() - 8usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_Z12alias_paramsP3BarP3Foo"]
    pub fn alias_params(a: Quux<Bar>, b: Quux<Foo>) -> Quux<Foo>;
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Quux_open0_Foo_close0",
    ][::std::mem::size_of::<Quux<Foo>>() - 8usize];
    [
        "Align of template specialization: Quux_open0_Foo_close0",
    ][::std::mem::align_of::<Quux<Foo>>() - 8usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_Z13default_parami"]
    pub fn default_param(x: ::std::os::raw::c_int) -> Quux<Foo>;
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Quux_open0_Foo_close0",
    ][::std::mem::size_of::<Quux<Foo>>() - 8usize];
    [
        "Align of template specialization: Quux_open0_Foo_close0",
    ][::std::mem::align_of::<Quux<Foo>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Quux_open0_Foo_close0",
    ][::std::mem::size_of::<Quux<Foo>>() - 8usize];
    [
        "Align of template specialization: Quux_open0_Foo_close0",
    ][::std::mem::align_of::<Quux<Foo>>() - 8usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_Z15trailing_returni"]
    pub fn trailing_return(x: ::std::os::raw::c_int) -> Quux<Foo>;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Holder {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Holder"][::std::mem::size_of::<Holder>() - 1usize];
    ["Alignment of Holder"][::std::mem::align_of::<Holder>() - 1usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_ZN6Holder6methodEi"]
    pub fn Holder_method(this: *mut Holder, x: ::std::os::raw::c_int) -> Quux<Foo>;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZNK6Holder12const_methodEii"]
    pub fn Holder_const_method(
        this: *const Holder,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    ) -> Quux<Bar>;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN6Holder13static_methodEi"]
    pub fn Holder_static_method(x: ::std::os::raw::c_int) -> Quux<Foo>;
}
impl Holder {
    #[inline]
    pub unsafe fn method(&mut self, x: ::std::os::raw::c_int) -> Quux<Foo> {
        Holder_method(self, x)
    }
    #[inline]
    pub unsafe fn const_method(
        &self,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    ) -> Quux<Bar> {
        Holder_const_method(self, x, y)
    }
    #[inline]
    pub unsafe fn static_method(x: ::std::os::raw::c_int) -> Quux<Foo> {
        Holder_static_method(x)
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Quux_open0_Foo_close0",
    ][::std::mem::size_of::<Quux<Foo>>() - 8usize];
    [
        "Align of template specialization: Quux_open0_Foo_close0",
    ][::std::mem::align_of::<Quux<Foo>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Quux_open0_Bar_close0",
    ][::std::mem::size_of::<Quux<Bar>>() - 8usize];
    [
        "Align of template specialization: Quux_open0_Bar_close0",
    ][::std::mem::align_of::<Quux<Bar>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Quux_open0_Foo_close0",
    ][::std::mem::size_of::<Quux<Foo>>() - 8usize];
    [
        "Align of template specialization: Quux_open0_Foo_close0",
    ][::std::mem::align_of::<Quux<Foo>>() - 8usize];
};
