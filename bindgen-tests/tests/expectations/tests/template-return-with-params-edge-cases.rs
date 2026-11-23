#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug)]
pub struct Foo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Box<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub ptr: *mut T,
}
impl<T> Default for Box<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type Quux<T> = *mut T;
unsafe extern "C" {
    #[link_name = "\u{1}_Z10box_resulti"]
    pub fn box_result(x: ::std::os::raw::c_int) -> Box<Foo>;
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z13nested_resultii"]
    pub fn nested_result(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    ) -> Box<Quux<Foo>>;
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Quux_open0_Box_open1_Foo_close1_close0",
    ][::std::mem::size_of::<Quux<Box<Foo>>>() - 8usize];
    [
        "Align of template specialization: Quux_open0_Box_open1_Foo_close1_close0",
    ][::std::mem::align_of::<Quux<Box<Foo>>>() - 8usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_Z19alias_of_box_resulti"]
    pub fn alias_of_box_result(x: ::std::os::raw::c_int) -> Quux<Box<Foo>>;
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z16const_arg_resulti"]
    pub fn const_arg_result(x: ::std::os::raw::c_int) -> u8;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct WithInlineBody {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of WithInlineBody"][::std::mem::size_of::<WithInlineBody>() - 1usize];
    ["Alignment of WithInlineBody"][::std::mem::align_of::<WithInlineBody>() - 1usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_ZN14WithInlineBody13declared_onlyEi"]
    pub fn WithInlineBody_declared_only(
        this: *mut WithInlineBody,
        x: ::std::os::raw::c_int,
    ) -> Quux<Foo>;
}
impl WithInlineBody {
    #[inline]
    pub unsafe fn declared_only(&mut self, x: ::std::os::raw::c_int) -> Quux<Foo> {
        WithInlineBody_declared_only(self, x)
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
        "Size of template specialization: Box_open0_Foo_close0",
    ][::std::mem::size_of::<Box<Foo>>() - 8usize];
    [
        "Align of template specialization: Box_open0_Foo_close0",
    ][::std::mem::align_of::<Box<Foo>>() - 8usize];
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
        "Size of template specialization: Box_open0_Foo_close0",
    ][::std::mem::size_of::<Box<Foo>>() - 8usize];
    [
        "Align of template specialization: Box_open0_Foo_close0",
    ][::std::mem::align_of::<Box<Foo>>() - 8usize];
};
